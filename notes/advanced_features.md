# Advaned Features
- Unsafe Rust: how to opt out of some of Rust's guarantees and take responsibility for manually upholding those guarantees
- Advanced traits: associated types, default type parameters, fully qualified syntax, supertraits, and the newtype pattern in relation to traits
- Advacned types: more about the newtype pattern, type aliases, the never type, and dynamically sized types
- Advanced functions and closures: function pointers and returning closures
- Macros: ways to define code that defines more code at compile time

## Unsafe Rust
- Rust has a second language hidden inside that doesn't enforce memory safety guarantees at compile time called unsafe Rust that works just like regular Rust
- Unsafe Rust exists because static analysis is conservative, when the compiler tries to determine whether or not code upholds the guarantee, it's better for it to reject some valid programs than to accept some invalid programs, although code may be okay, if Rust compiler lacks enough information to be conident, code ill be rejected, can use unsafe to tell the compiler to ignore the situation, if unsafe code is used incorrectly, problems can occur due to memory unsafety such as null pointer dereferencing
- Another reason Rust ha an unsafe version is that the underlying computer hardware is inherently unsafe, Rust needs to to allow unsafe code to do certain tasks and enables low-level systems programming, such as interacting with operating system or writing one, working with low-level systems programming is a goal of Rust

### Unsafe Superpowers
- To switch to unsafe Rust, can use `unsafe` keyword and then start a new block that holds the unsafe code, can take five actions in unsafe Rust that can't do in safe Rust, called unsafe superpowers, they include the ability to:
    - Dereference a raw pointer
    - Call an unsafe function or method
    - Access or modify a mutable static variable
    - Implement an unsafe trait
    - Access fields of a `union`
- `unsafe` doesn't turn off the borrow checker or disable any of Rust's other safety checks, if using a reference in unsafe code, will be checked, the `unsafe` keyword only gives access to these five features that are then not checked by the compiler for memory safety, will get some degree of safety inside of an unsafe block
- `unsafe` does not mean the code inside the block is necessarily dangerous or that it will definitely have memory safety problems, the intent is that as the programmer, will ensure the code inside an `unsafe` block will access memory in a valid way
- By requiring these five unsafe operations to be inside `unsafe` blocks, will know that errors related to memory safety must be within an `unsafe` block, `unsafe` blocks should be small
- To isolate unsafe code as much as possible, is best to enclose such code within a safe abstraction and provide a safe API, parts of the standard library are implemented as safe abstractions over unsafe code that has been audited, wrapping unsafe code in a safe abstraction prevents use of `unsafe` from leaking out into all the places to use functionality implemented with `unsafe` code, because a safe abstraction in safe

### Dereferencing a Raw Pointer
- Compiler ensures references are always valid, unsafe Rust has two new types called raw pointers that are similar to references, as with references, raw pointers can be immutable or mutable and are written as `*const T` and `*mut T` respectively, the asterisk isn't the dereference operator, it's part of the type name, in the context of raw pointers immutable means that the pointer can't be directly assigned after being dereferenced
- Different from references and smart pointers, raw pointers:
    - Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
    - Aren't guaranteed to point to valid memory
    - Are allowed to be null
    - Don't implement any automatic cleanup
- By opting out of having Rust enforce these guarantees, can give up guaranteed safety in exchange for the greater performance or the ability to interface with another language or hardware where Rust's guarantees don't apply
- Example: ```
        let mut num = 1;

        let r1 = &raw const num;
        let r2 = &raw mut num;```
- No `unsafe` keyword included in this code, can create raw pointers in safe code, can't dereference raw pointers outside an unsafe block
- Have created raw pointers by using the raw borrow operators: `&raw const num` creates a `*const i32` immutable raw pointer and `&raw mut num` creates `*mut i32` mutable raw pointerHave created raw pointers by using the raw borrow operators: `&raw const num` creates a `*const i32` immutable raw pointer and `&raw mut num` creates `*mut i32` mutable raw pointer, since these were created directly from a local variable, these particular raw pointers are valid but can't make that assumption about any raw pointer
- Can create a raw pointer whose validity can't be so certain of, using `as` to cast a value instead of using the raw borrow operators, can create a raw pointer to an abritrary location in memory doing this:
```
    let address = 0x12345usize;
    let r = address as *const i32;```
- Trying to use abritrary memory is undefed: there might be data at that address or there may not, the compiler might optimimze the code so there is no memory access, or program my terminate with a segmentation fault, usually, there is no good reason to write code like this especially in cases where raw borrow operator can be used instead, but it is possible
- Can create raw pointers in safe code, but can't dereference raw pointers and read the data being pointed to, can use the dereference operator `*` on a raw pointer but htis requires an `unsafe` block
- Example: ```
    let mut num = 1;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    println!("{r1:?}, {r2:?}");

    unsafe  {
        println!("{}, {}", *r1, *r2);
    }```
- Creating a pointer does no harm, only when trying to access the value that it points at, might end up dealing with an invalid value
- These are `*const i32` and `*mut i32` raw pointers that point to the same location in memory, where `num` is stored, if instead created an immutable and mutable reference to `num`, code wouln't have compiled because Rust's ownership rules don't allow a mutable reference at the same time as any immutable references, with raw pointers, can create a mutable pointer and an immutable pointer to the same location and cahnge the data through hte mutable pointer, potentially creating a data race
- One major use of raw pointers is interfacing with C code, another case is when building up safe abstractions that the borrow checker doesn't understand

### Calling an Unsafe Function or Method
- This type of operation is can be performed in an unsafe block that is calling unsafe functions, unsafe functions and methods look like regular functions and methods but they have an extra `unsafe` before the rest of the function, the `unsafe` keyword in this context indicates the function has requirements to uphold when calling it, since Rust can't guarantee these requirements, by calling an unsafe function within an `unsafe` block, saying that have read this function's documentation and take responsibility for upholding the function's contracts
- Example: ```
    unsafe {
        dangerous();
    }

unsafe fn dangerous() {}```
- Must call the `dangerous` function within a separate `unsafe` block, if trying to call `dangerous` without the `unsafe` block will get an error
- With the `unsafe` block, asserting to Rust that have read the function's documentation and understand how to use it properly, and have verified that the code fulfills the contract of the function
- To perform unsafe operations in the body of an unsafe function, still ned to use an `unsafe` block just as within a regular function and the compiler will warn if this is forgotten, this helps to keep `unsafe` blocks as small as possible

### Creating a Safe Abstraction over Unsafe Code
- Just because a function contains unsafe code doesn't mean need to mark the entire function as unsafe, can wrap unsafe code in a safe function, as in `split_at_mut` function from standard library which requires some usnafe code, safe method is defined on mutable slices, takes one slice and makes it two by splitting the slice at the index given an argument
- Example: ```
    let mut v = vec![1, 2, 3, 4, 5];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5]);```
- Can't implement the function using only safe Rust, an attempt may look something like this which won't compile
- Example: ```
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    (&mut values[..mid], &mut values[mid..])
}```
- This function gets the total length of the slice, then asserts that the index given as a parameter is within the slice by checking whether it's less than or equal to the length, the assertion means if passing in a value that is greater than the length to split the indx at, the function will panic before attempting to use that index, then returning two mutable slices in a tuple, one from the start of the original slice, to the `mid` index and another from `mid` to the end of the slice
- Rust's borrow checker can't tell that the two borrows are different parts of the slice, it only knows that this is borrowing from the same slice, borrowing from different parts is fundamentally okay because the two slices aren't overlapping, but Rust isn't smart enough to know this, when knowing the code is ok but Rust does not, use unsafe code
- This is the implementation using an `unsafe` block, a raw pointer, and some calls to unsafe functions to make the implementaton of `split_at_mut` work
- Example: ```
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}```
- Slices are a pointer to some data and the length of the slice, can use the `len` method to get the length of a slice and the `as_mut_ptr` method to access the raw pointer of a slice, because there is a mutable slice to `i32` values, `as_mut_ptr` returns a raw pointer with the type `*mut i32` which is stored in the variable `ptr`
- Kept the assertion that the `mid` index is wihtin the slice, then get to the unsafe code, the `slice::from_raw_parts_mut` function takes a raw pointer and a length, and creates a slice, can use it to create a slice that starts from `ptr` and is `mid` items long, can then call the `add` method on `ptr` with `mid` as an argument to get a raw pointer that starts at `mid`, and create a slice using that pointer and the remaining number of items after `mid` as the length
- The function `slice::from_raw_parts_mut` is unsafe because it takes a raw pointer and must trust that this pointer is valid, the `add` method on raw pointers is also unsafe because it must trust that the offset location is also a valid pointer, must place in `unsafe` block around calls to `slice::from_raw_parts_mut` and `add` so can call them, by looking at code and adding the assertion that `mid` must be less than or equal to `len`, can tell that all the raw pointers used within the `unsafe` block will be valid pointers to data within the slice, this is an acceptable and appropriate used of `unsafe`
- Don't need to mark the resultant `split_at_mut` function as `unsafe` and can call this function from safe Rust, have created a safe abstraction to the unsafe code with an implementation of the function that uses `unsafe` code in a safe way, since it creates only valid pointers from the data this function has access to
- In contrast, this use of `slice::from_raw_parts_mut` would likely crash when the slice is used, the code takes an arbitrary memory location and creates a slice 10,000 items long
- Example: ```
use std::slice;

let address = 0x01234usize;
let r = address as *mut i32;

let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 1000) };```
- Don't own the memory at this arbitrary location, there is no garuantee that the slice this code creates contains valid `i32` values, attemptign to use `values` as though it's a valid slice returns undefined behavior

### Using `extern` Functions to Call External Code
- Sometimes Rust code may need to interact with code written in another language, for this, Rust has keyword `extern` that facilitates the creation and use of a foreign function interface (FFI), an FFI is a way for a programming language to define functions and enable a different programming language to call those functions
- Here, this sets up an integration with the `abs` function from the C standard library, functions declared within `extern` blocks are generally unsafe to call from Rust code so `extern` blocks must also be marked with `unsafe`, the reason is that other languages don't enforce Rust's rules and guarantees and Rust can't check them, so responsibility falls on the programmer to ensure safety
- Example: ```
unsafe extern "C" {
    fn abs(input: i32) -> i32
}

unsafe {
    println!("abs val of -5 according to C: {}", abs(-5));
}```
- Within the `unsafe extern "C"` block, have listed the names and signatures of external functions from another language to call, the `"C"` part defines which application binary interface (ABI) the external function uses, the ABI defines how to call the function at the assembly level, the `"C"` ABI is the most common and follows the C programming language's ABI, info about all the ABIs Rust supports is available in the Rust Reference
- Every item declared within an `unsafe extern` block is implicitly unsafe, some FFI functions are safe to call, the `abs` function from C's standard library does not have any memory safety considerations and know it can be called with any `i32`, in cases like this, can use the `safe` keywrod to say that this specific function is safe to call even though it is in an `unsafe extern` block, after making that change, calling it no longer requires an `unsafe` block
- Example: ```
unsafe extern "C" {
    safe fn abs(input: i32) -> i32
}

println!("abs val of -5 according to C: {}", abs(-5));```
- Marking a function as `safe` does not inherently make it safe, it is like a promise made to rust that it is safe, still caller's responsibility to make sure the promise is kept

### Calling Rust Functions from Other Languages
- Can use `extern` to create an interface that allows other languages to call Rust functions, instead of creating a whole `extern` block, can add the `extern` keyword and specify the ABI to use just before the `fn` keyword for the relevant function, also need an `#[unsafe(no_mangle)]` annotation to tell the Rust compiler not to mangle the name of this function, mangling is when a compiler changes the name of a given function to a different name that contains more information for other parts of the compilation process but is less human readable, every compiler mangles names slightly differently, so for a Rust function to be nameable by other languages, must disable the Rust compiler's name mangling, this is unsafe because ther emight be name collisions across libraries without the built-in mangling, so it is programmer's responsibility to make sure the name chosen is safe to export without mangling
- Following function `call_from_c` is made accessible from C code after it's compiled to a shared library and linked from C
- Example: ```
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("calling a rust function from C");
}```
- The usage of `extern` requires `unsafe` only in the attribute, not on the `extern` block

### Accessing or Modifying a Mutable Static Variable
- Static variables are similar to constants, the names of static variables are in `SCREAMING_SNAKE_CASE` by convention, static variables can only store references with the `'static` lifetime, meaning the Rust compiler can figure out the lifetime and cannot annotate it explicitly, accessing an immutable static variable is safe
- Example: `static HELLO_WORLD: &str = "hello world";`
- Subtle difference between constants and immutable static variables is that values in a static variable have a fixed address in memory, using the value will always access the same data
- Constants are allowed to duplicate their data whenever they're used
- Static variables can be mutable
- Accessing and modifying mutable static variables is unsafe
- Example: ```
static mut COUNTER: u32 = 0;

/// SAFETY: Calling this from more than a single thread at a time is undefined behavior, must
/// guarantee that it is only called from a single thread at a time
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

/// SAFETY: This is only called from a single thread in `main`
unsafe {
    add_to_count(3);
    println!("COUNTER: {}", *(&raw const COUNTER));
}```
- Must only call `add_to_count` from one thread at a time
- As with regular variables, mutability is indicated with the `mut` keyword, any code that reads or writes from `COUNTER` must be within an `unsafe` block, this code works as expected because it's single threaded, having multiple threads access `COUNTER` would likely result in data races, so it is undefined behavior, need to mark entire function as `unsafe` and document the safety limitation so anyone calling the function knows that they are and are not allowed to do safely
- Whenever writing an unsafe function, it is idiomatic to write a commend staring with `SAFETY` and explaining what the caller needs to do to call the function safely, whenever performing an unsafe operation, t is idiomatic to write a comment, starting with `SAFETY` to explain how the safety rules are upheld
- Compiler won't allow creaitng references to a mutable static variable, can only access it via raw pointers, created with one of the raw borrow operators, that includes in cases where the reference is created invisibly as when it is used in the `println!`, the requirement that references to static mutable variables can only be created via raw pointers help make the safety requirements for using them more obvious
- With mutable data that is globally accessible, it's difficult to ensure there are no data races, which is why Rust considers mutable static variables to be unsafe, where possible, it's preferable to use the concurrency techniques and thread-safe smart pointers so the compiler checks that data access from different threads is done safely

### Implementing an Unsafe Trait
- Can use `unsafe` to implement an unsafe trait, a trait is unsafe when at least one of its methods has smoe invariant that the compiler cannot verify, can declare that a trait is `unsafe` by adding the `unsafe` keyword before `trait` and marking the implementation of the trait as `unsafe` too
- Example: ```
unsafe trait foo {
    // methods
}

unsafe impl Foo for i32 {
    // impl goes here
}```
- By using `unsafe  impl`, promising that will uphold the invariants that the compiler cannot verify
- As an example, with the `Sync` and `Send` marker traits, compiler implements these traits automatically if types are composed entirely of other types that implement `Send` and `Sync`, if implementing a type that does not contain `Send` or `Sync`, such as raw pointers and want to mark that type as `Send` or `Sync`, must use `unsafe`, Rust cannot verify that the type upholds the guarantees that it can be safely sent across threads or accessed from multiple threads, therefore, need to do those checks manually and indicate as such with `unsafe`

### Accessing Fields of a Union
- Can access fields of a union with `unsafe`
- A `union` is similar to a struct, but only one declared field is used in a particular instance at one time, unions are primiarily used to interface with unions in C code, accessing union fields is unsafe because Rust can't guarantee the type of the data currently being stored in the union instance

### Using Miri to Check Unsafe Code
- Can use Miri to check that unsafe code is actually safe and correct, Miri is an official Rust tool to check for undefined behavior, whereas borrow checker is a static tool that works at comple time, Miri is a dynamic tool that works at runtime, checks code by running program and detecting whether code is in violation of rules it understands about how Rust should work
- Using Miri requires a nightly build of Rust, can install both a nightly version of Rust and the Miri tool by using `rustup +nightly component add miri`, this does not change what version of Rust a project uses, only adds tool to system to use whenever, can run Miri on a project by typing `cargo +nightly miri run` or `cargo +nightly miri test`
- Miri doesn't catch everything that may be incorrect when writing unsafe code, it is a dynamic analysis tool, only catches problems with code that actually gets run, need to use in conjunction with good testing techniques to increase confidence about hte unsafe code that is written

### When to Use Unsafe Code
- Using `unsafe` to use one of the five superpowers just discussed isn't wrong or even frowned upon, but it is tricker to get `unsafe` code correct because the compiler cna't help uphold memory safety, when there is a reason to use `unsafe` code, can do so, and having the `explicit` unsafe annotation makes it easier to track down the source of problems when they occur, whenever writing unsafe code, can use Miri to help be more confident that the code written upholds Rust's rules

## Advanced Traits

### Associated Types
- Associated types connect a type plaeholder with a trait such that the trait method definition can use these placeholder types in their signatures, the implementor of a trait will specify the concrete type to be used instead of the placeholder type for the particular implementation, this way, can define a trait that uses some types without needing to know exactly what those types are until the trait is implemented
- One example of a trait with an associated type is the `Iterator` trait that the standard library provides, the associated type is named `Item` and stands in for the type of the values the type implementing the `Iterator` trait is iterating over
- Example: ```
pub trait Iterator {
    type Output;

    fn next(&mut self) -> Option<Self::Output>;
}```
- The type `Items` is a placeholder, and the `next` method's definition shows that it will return values of the type `Option<Self::Item>`, implementors of the `Iterator` trait will specify the concrete type for `Item` and the `next` method and will return an `Option` containing a value of that concrete type
- Associated types may seem similar to generics, in that the latter allows defining a function without specifying what types it can handle
- To look at differences between the two, will look at an implementation of the `Iterator` trait on a type named `Counter` that specifies the `Item` is type `u32`
- Example: ```
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {}
}```
- Potential `Iterator` trait implementation with generics: ```
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}```
- Difference is that when using generics, must annotate the types in each implementation since `Iterator<String> for Counter` or any other type, could have multiple implementations of `Iterator` for `Counter`, when a trait has a generic parameter, it can be implemented for a type multiple times, changing the concrete types if the generic type parameters each time, when using the `next` method on `Counter`, would have to provide type annotations to indicate which implementation of `Iterator` to use
- With associated types, don't need to annotate types since can't implement a trait on a type multiple times, with definition using associated types, can choose what the type of `Item` will be only once, since there can only be one `impl Iterator for Counter`, don't have to specify intention of an iterator of `u32` values everywhere to call `next` on `Counter`
- Associated tyeps also become part of the trait's contract, implementors of the trait must provide a type to stand in for the associated type placeholder, associated types often have a name to describe how the type will be used, and documenting the associated type in the API documentation is good practice

### Default Generic Type Parameters and Operator Overloading
- When using generic type parameters, can specify a default concrete type for the generic type to eliminate the need for implementors of the trait to specify a concrete type if the default type works, can specify a default type when declaring a generic type with the `<PlaceholderType=ConcreteType>` syntax
- This technique is useful when with operator overloading, when customizing the behavior of operators such as `+`
- Rust doesn't allow creating new operators or overloading arbitrary operators, but can overload the operators and corresponding traits in `std::ops` by implementing the traits associated with the operator, can overload the `+` operator to add two `Point` instances together, can do this by implemeting the `Add` trait on a `Point` struct
- Example: ```
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });
}```
- The `add` method adds the `x` values of two `Point` instances and the `y` values to create a new `Point`, hte `Add` trait has an associated type named `Output` that determines the type returned from the `add` method
- The default generic type in this code is within the `Add` trait 
- Example: ```
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}```
- This is a trait with one method and an associated type, the new part is `Rhs=Self`, this syntax is called a default type parameters, the `Rhs` generiic type parameter is short for right-hand side, and defines the type of the `rhs` parameter in the `add` method, to not specify a concrete type for `Rhs` when implementing the `Add` trait, the type of `Rhs` will default to `Self` which will be the type implementing `Add` on
- When implementing `Add` for `Point`, used the default for `Rhs` since wanted to add two `Point` instances, here is an example of implementing the `Add` trait where customizing the `Rhs` rather than using the default
- Two structs, `Milimeteres` and `Meteres`, holding values in different units, this thin wrapping of an existing type in another struct is known as the newtype pattern, want to add values in milimeters to values in meters and have the implementation of `Add` do the conversion correctly, can implement `Add` for `Milimeters` with `Meters` as the `Rhs`
- Example: ```
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}```
- To add `Milimeters` and `Meters`, can specify `impl Add<Meters>` to set the value of the `Rhs` type parameter instead of using the default of `Self`
- Default type parameters are used in two ways:
    - To extend a custom type without breaking existing code
    - To allow customization in specific cases most users won't need
- The standard library's `Add` trait is an example of the second purpose, can add two like types, but the `Add` trait provides the ability to customize beyond that, using a default type parameter, in the `Add` trait definition means don't have to specify the extra parameter most of the time, in other words, a bit of implementation boilerplate isn't needed, making it easier to use the trait
- The first purpose is similar to the second but in reverse, to add a type parameter to an existing trait, can give it a default to allow extension of the functioanlity of the trait without breaking the existing implementation code

### Disambiguating Between Methods with the Same Name
- Nothing in Rust prevents a trait from having a method with the same name as another trait's method, nor does Rust prevent from implementing both traits on one type, it's also possible to implement a method directly on the type with the same name as methods from traits
- When calling methods with the same name, will need ot tell Rust which oen to use, here have defined two traits, `Pilot` and `Wizard`, both have a method called `fly`, then implement both traits on a type `Human` that already has a method called `fly` implemented on it, each `fly` method does something different
- Example: ```
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("called pilot on human");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("called wizard on human");
    }
}

impl Human {
    fn fly(&self) {
        println!("called fly on human");
    }
}```
- When calling `fly` on an instance of `Human`, the compiler defaults to calling the method directly implemented on that type
- Example: ```
fn main() {
    let person = Human;
    person.fly();
}```
- Running this code will print `called fly on human`, showing that Rust called the `fly` method implemented on `Human` directly
- To call the `fly` methods from either the `Pilot` trait or the `Wizard` trait, need to use more explicit syntax to specify which `fly` method is intended
- Example: ```
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();```
- Specifying the trait name before the method clarifies to Rust which implementation of `fly` to call, could also write `Human::fly(&person)`, which is equivalent to `person.fly()`, but this is a bit longer to write if there is no need to disambiguate
- If there was two types that both implement one trait, Rust could figure out which implementation of a trait to use based on the type of `self`
- However, associated functions that are not methods don't have a `self` parameter, when there are multiple types or traits that define non-method functions with the same function name, Rust doesn't always know which type to use unless using fully qualified syntax, here, have created a trait for an animal shelter that wants to name all baby dogs Spot, can make an `Animal` trait with an associated non-method function `baby_name`, the `Animal` trait is implemented for the struct `Dog`, on which also provide an associated non-method function `baby_name` directly
- Example: ```
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("a baby dog is called a {}", Dog::baby_name());
}```
- Have implemented the code for naming all puppies Spot in the `baby_name` associated function that is defined on `Dog`, the `Dog` type also implements the trait `Animal`, which describes characteristics that all animals have, baby dogs are called puppies, this is expressed in the implementation of the `Animal` trait on `Dog` in the `baby_name` function associated with the `Animal` trait
- In `main`, can call the `Dog::baby_name` function which calls the assocaited function defined on `Dog` directly
- If changing the code to use `Animal::baby_name()`, get a compilation error, since `Animal::baby_name` doesn't have a `self` parameter and there could be other types that implement the `Animal` trait, Rust can't figure out which implementation of `Animal::baby_name` to use
- To disambiguate and tell Rust to use the implementation of `Animal` for `Dog` as opposed to the implementation of `Animal` for some other type, need to use fully qualified syntax
- Example: `println!("a baby dog is called a {}", <Dog as Animal>::baby_name());`
- Need to provide Rust with a type annotation within the angle brackets, which indicates to call the `baby_name` method from the `Animal` trait as implemented on `Dog` by saying to treat the `Dog` type as an `Animal` for this function call
- In general, fully qualified syntax is as follows: `<Type as Trait>::function(recevier_if_method, next_arg, ...);`
- For associated functions that aren't methods, there would not be a receiver, there would only be the list of other arguments, could use fully qualified syntax everywhere that methods and functions can be called, however, allowed to omit any part of this syntax that Rust can figure out from other multiple implementations that use the same name and Rust needs help to identify which implementation to call

### Using Superatraits
- Somtimes, may write a trait definition that depends on another trait: for a type to implement the first trait, want to require that type to also implement a second trait, would also do this so that trati definition can make use of the associated items of the second trait, the trait that the trait definition is relying on is called a supertrait of the trait
- To make an `OutlinePrint` trait with an `outline_print` method that will print a given value formatted so that it's framed in asterisks, that is given a `Point` struct that implements the standard library trait `Display` to result in `(x, y)`, when calling `outline_print` on a `Point` instance that has `1` for `x` and `3` for `y`, it should print it in a box of asterisks
- In the implementation of the `outline_print` method, want ot use the `Display` trait's functionality, neeed to specify that the `OutlinePrint` trait will work only for types that also implement `Display` and provide the funcionality that `OutlinePrint` needs, can do this in the trati definition by specifying `OutlinePrint: Display`, this is similar to adding a trait bound to the trait
- Example: ```
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string;
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 4));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4))
    }
}```
- Since have specified that `OutlinePrint` requires the `Display` trait, can use the `to_string` function that is automatically implemented for any type that implements `Display`, if trying to use `to_string` without adding a colon and specifying the `Display` trait after the trait name, would get an error saying that no method named `to_string` was found for the type `&Self` in the current scope
- When implementing `OutlinePrint` on a type that doesn't implement `Display`, such as the `Point` struc, will get an error saying that `Display` is not implemented
- To fix this, implement `Display` on `Point` and satisfy the constraint that `OutlinePrint` requires
- Example: ```
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}```
- Then, implementing the `OutlinePrint` trait on `Point` will compile successfully, and can call `outline_print` on a `Point` instance to display it within an outline of asterisks

### Using the Newtype Pattern to Implement External Traits on External Types
- The orphan rule states that only allowed to implement a trait on a type if either the trait or the type or both are local to the crate, can get around this restriction with the newtype pattern, which involves creating a new type in a tuple struct, the tuple struct will have one field and be a thin wrapper around the type for which to implement a trait, then the wrapper type is local to the crate and can implement the trait on the wrapper, Newtype is a term that originates from Haskell, there is no runtime performance from using this pattern and the wrapper type is elided at compile time
- As an example, to implement `Display` on `Vec<T>`, which the orphan rule prevents from doing directly because the `Display` trait and the `Vec<T>` type are defined outside the crate, can make a `Wrapper` struct that holds an instance of `Vec<T>` then can implement `Display` on `Wrapper` and use the `Vec<T>` value
- Example: ```
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hi"), String::from("hello")]);
}```
- The implementation of `Display` uses `self.0` to access the inner `Vec<T>` because `Wrapper` is a tuple struct and `Vec<T>` is the item at index 0 in the tuple, then can use the functionality of the `Display` trait on `Wrapper`
- The downside of using this technique is that `Wrapper` is a new type, so it doesn't have the methods of the value it's holding, would have to implement all the methods of `Vec<T>` directly on `Wrapper` such that the methods delegate to `self.0` which would allow treating `Wrapper` exactly like the `Deref` trait on the `Wrapper` to return the inner type would be a solution, if not wanting the `Wrapper` type to have all the methods of the inner type, to restrict the `Wrapper` type's behavior, would have to implement just the wanted methods manually
- This newtype pattern is also useful even when traits are not involved
