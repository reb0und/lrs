# Smart Pointers
- A pointer is a general concept for a variable that contains an address in memory that refers to or points at some other data, the mot common type of pointer in Rust is a reference
- References are indicated by the `&` symbol and borrow the value they point to, they don't have any special capabilities other than referring to data, and they have no overhead
- Smart pointers are data structures that act like a pointer but also have additional metadata and capabilities and capabilities, smart pointers originated in C++ and exist in other languages as well
- Rust has a variety of smart pointers defined in the standard library that provide functionality beyond that provided by references
- Examples of smart pointers include a reference counting smart pointer type that enable data to have multiple owners by keeping track of the number of owners and when no owners remain, cleaning up the data
- Rust, with ownership and borrowing, has an additional difference between references and smart pointers: while references borrow data, in many cases smart pointers own the data they point to
- A few smart pointes include `String` and `Vec<T>`, both of these types are smart pointers because they own some memory and allow manipulation of this memory, they also have metadata and extra capabilities or guarantees, `String` stores its capacity as metadata and has the extra ability to ensure its data will always be valid UTF-8
- Smart pointers are usually implemented using structs, unlike an ordinary struct, smart pointers implement the `Deref` and `Drop` traits
   - The `Deref` trait allows for an instance of the smart pointer struct to behave like a reference so code can work with either references or smart pointers
   - The `Drop` trait allows customization of code that's run when an instance of the smart pointer goes out of scope
- Many libraries have existing smart pointers, can even write custom ones
- Most common smart pointers in the standard library:
   - `Box<T>`, for allocating on the heap
   - `Rc<T>`, a reference counting type that enables multiple ownership
   - `Ref<T>` and `Ref<T>` and `RefMut<T>` accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time
- Interior mutability pattern is where an immutable type exposes an API for mutating an interior value
- Reference cycles can leak memory but are preventable

## Using `Box<T>` to Point to Data on the Heap
- A box, type of `Box<T>`, allows the storage of data on the heap rather than the stack
- What remains on the stack is the pointer to the heap data
- Boxes don't have performance overhead, other than storing their data on the heap instead of on the stack, but don't have many extra capabilites either
- Used most in these situations
   - When a type whose size can't be known at compile time and want to use a value of that type in a context that requires an exact size
   - When there is a large amount of data and trying to transfer ownership but ensure the data won't be copied when doing so
   - When intending to own a value but only care that it's a type that implements a particular trait than being of a specific type
- Transferring ownership of a large amount of data can take a long time because the data is copied around on the stack, to improve performance in this situation, can store the large amount of data on the heap in a box, then, only the small amount of pointer data is copied around on the stack, while data it references stays in one place on the heap, third case is known as a trait object

### Using `Box<T>` to Store Data on the Heap
- Example: `let b = Box::new(5);`
- Here, a variable `b` has the value of a `Box` that points to the value `5`, which is allocated on the heap, can access the data in the box similarly to if it was on the stack, just like any owned value, when a box goes out of scope, as `b` does at the end of `main`, it will be deallocated, the deallocation happens both for the box (stored on the stack) and the data it points to (stored on the heap)
- Putting a single value on the heap isn't very useful, won't use boxes by themselves very often, having values like a single `i32` on the stack, where they're stored by default is more appropriate in most situations

### Enabling Recursive Types with Boxes
- A value of a recrusive type can have another value of the same type as part of itself
- Recursive types pose an issue because Rust needs to know at compile time ho wmuch space a type takes upm however the nesting of values of recursive types could theoretically continue infinitely, so Rust can't know how much space the value needs
- Since boxes have a known size, can enable recursive types by inserting a box in the recursive type definition
- An example of a recursive type: the cons list is a data type commonly found in functional programming languages that is straightforward except for the recursion

#### More Information About the Cons List
- A cons list is a data structure that comes from Lisp and its dialects, is made up of nested pairs, and is the Lisp version of a linked list, name comes from the `cons` function (short for construct function) in Lisp that constructs a new pair from its two arguments, by calling `cons` on a pair consiting of a value and another pair, can construct cons lists made up of recursive pairs
   - Cons list containing `1, 2, 3`: `(1, (2, (3, Nil)))`
- Each item in a cons list contains two elements, the value of the current item and the next item, the last item in the list contains only a value called `Nil` without a next item, a cons list is produced by recursively calling the `cons` function, the canonical name to denote the base case of the recursion, not the same as an invalid or absent value
- Cons list is not commonly used, most of time is better to use `Vec<T>`
- Enum definition for a cons list: ```
enum List {
    Cons(i32, List),
    Nil,
}```
- This won't compile because a list does not have a known size
- Using the `List` type to store the list `1, 2, 3` would look like `let list = Cons(1, Cons(2, Nil));`
- The first `Cons` value holds `1` and another `List` value, this `List` value is another `Cons` value that holds `2` and a `List` value which is `Nil`, the non recursive variant signals the end of the list
- When attempting to compile this will receive error that recursive type `List` has infinite size, the reason `List` is defined with a recursive variant is that it holds another value of itself directly, as a result, Rust cannot figure out how much space it needs to store a `List` value

#### Computing the Size of a Non-Recursive Type
= Given a message enum: ```
enum Message {
   Quit,
   Move { x: i32, y: i32 },
   Write(String),
}```
- To determine how much space to allocate for a `Message` valye, Rust goes through each of the variants to see which variant needs the most space, because only one variant will be used, the most space a `Message` value will need is the space it would take to store the largest of its variants
- When Rust tries to determine how much space a recursive type like the `List` enum needs, the compiler starts by looking at the `Cons` variant which holds a value of type `i32` and a value of type `List`, to figure out how much memory the `List` type needs, the compiler lokos at the variants, starting with the `Cons` variant, the `Cons` variant is of type `i32` and a valye of type `List`, and this process continnues infinitely

#### Using `Box<T>` to Get a Recursive Type with a Known Size
- Rust suggests inserting some indirection to break the recursive cycle
- Indirection means that instead of storing a value directly, should change the data structure to store the value indrectly by storing a pointer to the value instead
- Since `Box<T>` is a pointer, Rust always knows how much space a `Box<T>` needs, a pointer's size does not change based on the amount of data it's pointing to, this means a `Box<T>` can be placed inside the `Cons` variant instead of another `List` value directly, the `Box<T>` will point to the next `List` value that will be on the heap rather than inside the `Cons` variant, conceptually, there still is a list created with lists holding other lists, but this implementation is now more like placing hte items next to one another rather than inside one another
- Example: ```
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
}```
- The `Cons` variant needs the size of an `i32` plus ths space to store the box's pointer data
- The `Nil` variant stores stores no values so it needs less space than the `Cons` variant
- Now know that any `List` value will take up the size of an `i32` plus the size of a box's pointer data
- By using a box, this breaks the infinite recursive chain, so the compiler cna figure out the size it needs to store a `List` value
- Boxes provide only the indirection and heap allocation, they don't have any other special capabilities, like those with other smart pointer types, they also don't have the performance overhead these special capabilities incur, so they can be useful in cases like the cons list where indirection is the only feature needed
- The `Box<T>` type is a smart pointer because it implements the `Deref` trait, which allows `Box<T>` values to be treated like references
- WHen a `Box<T>` value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the `Drop` trait implementation

## Treating Smart Pointers Like Regular References with `Deref`
- Implementing the `Deref` trait, allows customization of the behavior of the dereference operator `*`, by implementing `Deref` in such a way that a smart pointer can be treated like a regular reference, can write code that operates on references and use that code with smart pointers too

### Following the Pointer to the Value
- A regular reference is a type of pointer, one way to think about a pointer is an arrow to a value stored elsewhere
- Example: ```fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}```
- `x` holds an `i32` value `5`, `y` is a reference to `x`, have to use `*y` to follow the reference to the value it's pointing to so the compiler can compare the actual value, once `y` has been dereferenced, access it gained to the integer value `y` points to
   - Cannot compare a number and a reference to a number

### Using `Box<T>` like a Reference
- Can rewrite code to use a `Box<T>` instead of a reference, the dereference operator used on `Box<T>` functions in the same way as the dereference operator used on the reference from earlier
- Example: ```
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
} Example: ```
- Main difference is `y` is set to be an instance of a box pointing to a copied value of `x` rather than a reference pointing to the value of x
- Can dereference the operator to follow the box's pointer in the same way as when `y` was a reference

### Defining a Smart Pointer
- The `Box<T>` type is ultimately defined as a typle struct with one element
- Example: ```
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}```
- Have defined a struct named `MyBox` and declared a generic parameter `T` since, want the type to hold values of any type, the `MyBox` type is a tuple struct with one element of type `T`, the `MyBox::new` function takes on parameter of type `T` and returns a `MyBox` instance that holds the value passed in

### Implementing the `Deref Trait`
- To implement a trait, need to provide implementations for the trait's required mehtods, the `Deref` trait requires implementation of the `deref` method that borrows `self` and returns a reference to the inner data
- Example: ```
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}```
- The `type Target = T` defines an associated type for the `Deref` trait to use, associated types are a slightly different way of declaring a generic type parameter
- Body of `deref` mehtod is filled with `&self.0` so `deref` returns a reference to the value to access with the `*` operator
   - `.0` accesses the first value in a tuple struct
- Without the `Deref` trait, the compiler can only dereference `&` references, the `deref` method gives the compiler the ability to take a value of any type that implements `Deref` and call the `deref` method to get an `&` reference that it knows how to dereference
- When entering `*y`, behind the scenes, Rust actually runs this code `*(y.deref())`
- Rust substitutes the `*` operator with a call to the `deref` method, and then a plain dereference, so there is no need to think about whether to call the `deref` method, this Rust feature allows for writing code that functions identically whether there is a regular reference or a type that implements `Deref`
- The reason the `deref` method returns a reference to a value, and that the plain dereference outside the parentheses in `*(y.deref())` is still necessary, has to do with the ownership system, if the `deref` method returns the value directly instead of a reference to the value, the value would be moved out of `self`, don't want to take ownership of the inner value inside `MyBox<T>` in this case or in most cases when using the dereference operator
- Note that the `*` operator is replaced with a call to the `deref` method and then a call to the `*` operator just once, each time `*` is used in code, because of the substitution of the `*` operator does not recurse infinitely, end up with data type of `i32`

### Implicit Deref Coercions with Functions and Methods
- Deref coercion converts a reference to a type that implements the `Deref` trait into a reference to another type
- For example, deref coercion can convert `&String` to `&str` because `String` implements the `Deref` trait such that it returns `&str`, deref coercion is a convenience Rust performs on arguments to functions and methods and works only on types that implement the `Deref` trait, happens automatically when passing a reference to a particular type's value as an argument to a function or method that doesn't match the function or method definition
- A sequence of calls to the `deref` method converts the type provided into the type the parameter needs
- Deref coercion was added to Rust so writing functions and metod calls don't need to add as many explicit referencs with `&` and `*`, the deref coercion feature also allows writing more code that can work for either references or smart pointers
- Given a `Box<String>` can call a function that takes a `&str` with the `Box<String>` as a parameter since `Deref` implementation of `String` results in a `&str`, deref coercion makes it possible to call a function that takes a string slice with as an argument with a reference to a value of type `MyBox<String>`
- Since the `Deref` trait has been implemented on `MyBox<T>`, Rust can turn `&MyBox<String>` into `&String` by calling `deref`, the standard library provides an implementation of `Deref` on `String` that returns a string slice, Rust calls `deref` again to turn the `&String` into `&str`, which matches the function's definition
- Can take a string slice of a `String` with `[..]` that is equal to the whole string, deref coercion allows Rust to automatically handle these conversions
- When the `Deref` trait is defined for the types involved, Rust will analyze the types and use `Deref::deref` as many times as necessary to get a reference to match the parameter's type, the number of times `Deref::deref` needs to be inserted is resolved at compile time, so there is no runtime for taking advantage of deref coercion

### How Deref Coercion Interacts with Mutability
- Similar to how the `Deref` trait is used to override the `*` operator on immutable references, can use the `DerefMut` trait to override the `*` operator on mutable references
- Rust does deref coercion when it finds types and trait implementations in three cases:
1. From `&T` to `&U` when `T: Deref<Target=U>`
2. From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
3. From `&mut T` to `&U` when `T: Deref<Target=U>`
- The first two cases are the same except that the second implements mutability
- First case states that given a `&T` and `T` implements `Deref` to some type `U`, can get a `&U` transparanetly, second case states that the same deref coercion happens for mutable references
- Third cases states Rust will also coerce a mutable reference to an immutable one, but reverse is not possible

## Running Code on Cleanup with the `Drop` Trait
- The second trait to the smart pointer pattern is `Drop`, allows for customization on what happens when a value is about to go out of scope, can provide an implementation for the `Drop` trait on any type and that code can be used to release resources like files or network connections
- For example, when `Box<T>` is dropped, it will deallocate the space on the heap that the box points to
- In Rust, can specify that a particular bit of code be run whenever a value goes out of scope, and the compiler will insert this code automatically, as a result don't need to be careful about placing cleanup code everywhere in a program that an instance of a particular type is finished with
- Can specify the code to run when a value goes out of scope by implementing the `Drop` trait, the `Drop` trait allows for implementation of one method named `drop` that takes a mutable reference to `self`, to see when Rust calls `drop`
- `Drop` trait is included in the prelude, no need to bring it into scope, can implement the `Drop` trait on `CustomSmartPointer` and provide an implementation for the `drop` method that includes any logic for when an instance of a type goes out of scope
- No need to call the `drop` method explicitly, automatically called when value goes out of scope, variables are dropped in the reverse order of their creation
- Disabling `drop` usually isn't necessary, whole point of the `Drop` trait is that it's taken care of automatically
   - When smart pointers manage locks, may want to force the `drop` method that releases the lock so that other code in the same scope can acquire the lock
   - Rust does not allow manually calling the `Drop` trait's `drop` method manually, instead need to call the `std::mem::drop` function provided by the standard library to force a valye to be dropped before the end of its scope
   - Rust does not allow a call of `drop` explicitly since Rust would still automatically call `drop` on the value at the end of `main`, this would cause a double free error because Rust would be trying to clean up the same value twice
   - Can't disable automatic insertion of `drop` when a value goes out of scope, can't call the `drop` method explicitly, can force a value to be cleaned up early using the `std::mem::drop` function, which is different from the `drop` method in the `Drop` trait, can call it by passing an argument the value to force-drop
   - Can use code specified in a `Drop` trait implementation in many ways to make cleaniup convenient and safe, for instance, can make a memory allocator, with the `Drop` trait and Rust's ownership system, don't need to remember to clean up because Rust does so automatically, also don't need to worry about problems resulting from accidentally cleaning up values still in use, the ownership system makes sure references are always valid and also ensures that `drop` gets called only once when the value is no longer being used

## `Rc<T>`, the Reference Counted Smart Pointer
- In the majority of cases, ownership is clear, know exactly which variable owns a given value
- There are cases when a single value might have multiple owners, in graph data structures, multiple edges may point to the same node, and that node is conceptually owned by all of the edges that point to it, a node shouldn't be cleaned up unless it doesn't have any edges pointint to it and so has no owners
- Have to enable multiple ownership explicitly by using the Rust type `Rc<T>` which is an abbreviation for reference counting, the `Rc<T>` type keeps track of the number of references to a value to determine whether or not the value is still in use, if there are zero references to a value, the value can be cleaned up without any references becoming invalid
- When using the `Rc<T>` type when wanting to allocate soem data on the heap for multiple parts of the program to read and can't determine at compile time which part will finish using the data last, `Rc<T>` is only for use in single-threaded scenarios

### Using `Rc<T>` to Share Data
- Example: ```enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}```
- Could change definition of `Cons` to hold references instead, but then would need to specify lifetime parameters, by specifying lifetime parameters, would be specifying that every element in the list would live at least as long as the entire list, this is not the case for all scenarios
- Instead, can change the definition of `List` to use `Rc<T>` in place of `Box<T>`, each `Cons` variant will now hold a value and an `Rc<T>` pointing to a `List`, when creaitng `b`, instead of taking ownership of `a`, will clone the `Rc<List>` that `a` is holding, increasing the number of references from one to two and letting `a` and `b` store the data in that `Rc<List>`, can also clone `a` when creating `c`, increasing the number of references from two to three, every time `Rc::clone` is called, the reference count to the data within `Rc<List>` will increase, and the data won't be cleaned up unless there are zero references to it
- Exampe: ```enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}```
- Need to bring `Rc<T>` into scope using `use std::rc::Rc;` because it's not in the prelude, here the list holding 5 and 10 is created and stored in a `Rc<List>` in `a`, then creating `b` and `c` calling the `Rc::clone` function and pass a reference to the `Rc<lIST>` IN `a` as an argument
- Could have called `a.clone()` rather than `Rc::clone(&a)`, but Rust's convention is to use `Rc::clone` in this case, the implementation of `Rc::clone` doesn't make a deep copy of all the data like most implementions of `clone` do
- The call to `Rc::clone` only increments the reference count, which doesn't take much time, deep copies of data can take a lot of time, by using `Rc::clone` for reference counting, can visually distinguish between deep-copy kinds of clones and the kinds of clones that increase the reference count

### Cloning an `Rc<T>` Increases the Reference Count
- Can get the reference count by calling the `Rc::strong_count` function, this function is named `strong_count` rather than `count` because the `Rc<T>` type also has a `weak_count`
- No function to decrease the count, the implementation of the `Drop` trait decreases the reference count automatically when an `Rc<T>` value goes out of scope
- When all references go out of scope at the end of `main`, the count is 0 and the `Rc<List>` is cleaned up completely, using `Rc<T>` allows a single value to have multiple owners, and the count ensures that the value remains valid as long as any of the owners still exist
- Via immutable references, `Rc<T>` allows sharing of data between multiple parts of a program for reading only, if `Rc<T>` allowed multiple mutable references, this might violate one of the borrowing rules: multiple mutable borrows to the same place can cause data races and inconsistencies, can use `RefCell<T>` type in conjunction with `Rc<T>` to work with this immutability restriction

## `RefCell<T>` and the Interior Mutability Pattern
- Interior mutability is a design pattern in Rust that allows the mutation of data when there are immutable references to that data
   - Normally, this is disallowed by the borrowing rules, to mutate data, the pattern uses `unsafe` code inside a data structure to bend Rust's usual rules that govern mutation and borrowing, unsafe code indicates to the compiler that the rules are checked manually instead of relying on the compiler to check them 
- Can use types that use the interior mutability pattern only when it can be ensured that the borrowing rules will be followed at runtime, even though the compiler still can't guarantee this, the `unsafe` code is wrapped in a safe API, and the outer type is still immutable

### Enforcing Borrowing Rules at Runtime with `RefCell<T>`
- Unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the data it holds
- At any given time, can either have one mutable reference or any number of immutable references (but not both)
- References must always be valid
- With references and `Box<T>`, the borrowing rules are enforced at compile time, but with `RefCell<T>`, they are enforced at runtime
- With references, if these rules are broken, will get a compiler error, with `RefCell<T>`, program will panic and exit
- Advantages of checking borrowing rules at compile time are that errors are caught sooner in development process, no impact on runtime performance because all the analysis is completed beforehand, for these reasons, checking the borrowing rules at compile time is the best choice in the majority of cases, which is why this is Rust's default
- The advantage of checking the borrowing rules at runtime instead is that certain memory-safe scenarios are then allowed, where they would've been prevented by compile time checks, static analysis, like the Rust compiler, is inherently conservative, some properties are impossible to detect by analyzing the code
- Since some analysis is impossible, if the Rust compiler can't be sure the code compiles with the ownership rules, it might reject a correct program, in this way, it's conservative
- If Rust accepted an incorrect program, users wouldn't be able to trut in the guarantees Rust makes, however if Rust rejects a correct program, the programmer will be inconvenienced, but nothing troubling can occur
- The `RefCell<T>` type is useful the borrowing rules seem to be followed but the compiler is unable to understand and guarantee that
- Similar to `Rc<T>`, `RefCell<T>` is only for use in single-threaded scenarios and will give a compile-time error if attempted to be used in a multithreaded context 
- Reasons to choose `Box<T>`, `Rc<T>`, or `RefCell<T>`:
   - `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>` have single owners
   - `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime
   - Because `RefCell<T>` allows mutable borrows cheked at runtime, can mutate the value inside `RefCell<T>` even when the `RefCell<T>` is immutable
- Mutating the value inside an immutable value is the interior mutability pattern

### Interior Mutability: A Mutable Borrow to an Immutable Value
- Consequence of the borrowing rules is that given an immutable value, can't borrow it mutably
- Despite this, there are situations in which it would be useful for a value to mutate itself in its methods but appear immutable to other code
- Code outside the value's methods would not be able to mutate the value, using `RefCell<T>` is one way to gain the ability to have interior mutability, but `RefCell<T>` doesn't get around the borrowing rules completely: borrow checker in compiler allows this interior mutability, and the borrowing rules are checked at rutime instead, if rules are violated, program will `panic!` instead of compiler error

#### A Use Case for Interior Mutability: Mock Objects
- Sometimes during testing, may need to use a type in place of another type in order to observe particular behavior and assert that it's implemented correctly, placeholder type is called a test double, can stand in for other types when runnning tests
- Mock objects are specific types of test doubles that record what happens during a test so the correct actions can be asserted having taken place
- Rust doesn't have obkects in the same sense as other languages have obkects and Rust doesn't have mock object functionality built into the standard library, however, can create a struct that will serve the same purpose as a mock object
- Can create a library that tracks a value against a maximum value and sends messages based on how close to the maximum value the current valu is, library could be used to keep track of a user's quota for API calls
- Library will only provide functionality of tracking how close to the maximum a value is and what the messages should be at what times, applications using library are expected to provide the mechanism for sending the messages, library doesn't need to know the detail, all it needs is something that implements a trait called `Messenger`
- Example: ```
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: Quota exceeded");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: Used up over 75% of quota");
        }
    }
}```
- Important part of this code is that the `Messenger` trait has one method called `send` that takes an immutable reference to `self` and the text of the message, this trait is the interface the mock object needs to implemenet so that mock can be used in the same way a real object is
- Other important part is that the behavior of `set_value` method on the `LimitTracker` needs to be tested
   - Can change what is passed in for the `value` parameter, but `set_value` doesn't return anything to make assertions on
   - Want to be able to say that if creating a `LimitTracker` with something that implements the `Messenger` trait and a particular value for `max`, when passing different numbers for `value`, the messenger is told to send the appropriate messages
- Example: ```
#[cfg(test)]
mod tests {
    use super::*;
    
    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 10);

        limit_tracker.set_value(8);

        assert_eq!(mock_messenger.sent_messages, vec!["Warning: Used up over 75% of quota"]);
    }
}```
- This test code defines a `MockMessenger` struct that has a `sent_messages` field with a `Vec` of `String` values to keep track of the messages it's told to send, also have defined an associated function `new` to make it convenient to create new `MockMessenger` values that start with an empty list of messages, then implemented `Messenger` trait for `MockMessenger` so can give a `MockMessenger` to a `LimitTracker`, in the definition of the `send` method, the message passed in a parameter is taken and stored in the `MockMessenger` list of `sent_messages`
- In the test, this tests what happens when the `LimitTracker` is told to set `value` to something more than 75 percent of the `max` value, this first creates a `MockMessenger` which will store an empty list of messages, then creates a `LimitTracker` and is given a reference to the new `MockMessenger` and a `max` of `10`, the `set_value` method is called on the `LimitTracker` with a value of `8` (more than 75 percent of 100), then must assert the list of messages that the `MockMessenger` is keeping track of should now have message in it
- Can't modify the `MockMessenger` to keep track of the messages since the `send` method takes an immutable reference to `self`, also can't take the suggestion from error text to use `&mut self` in both the `impl` method and the `trait` definition, do not want to change the `Messenger` trait soley for sake of testing, instead want to find a way to make test code work correctly with existing design
- This is a situation in which interior mutability can help, can store the `sent_messages` within a `RefCell<T>`, and then the `send` method will be able to modify `sent_messages` to store the messages seen
- Example: ```
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 10);

        limit_tracker.set_value(8);

        assert_eq!(mock_messenger.sent_messages, vec!["Warning: Used up over 75% of quota"]);
    }
}```
- The `sent_messages` field is now of type `RefCell<Vec<String>>` instead of `Vec<String>`, `new` function creates a new `RefCell<Vec<String>>` instance around the empty vector, the `send` method's first parmeter is still an immutable borrow of `self` which matches the trait definition, can call `borrow_mut` on the `RefCell<Vec<String>>` in `self.sent_messages` to get a mutable reference to the value inside the `RefCell<Vec<String>>` which is the vector, can call `push` on the mutable reference to the vector to keep track of the messages sent during the test, last change to make is in the assertoin, to see how many items are in the inner vector, need to call `borrow` on the `RefCell<Vec<String>>` to get an immutable reference to the vector

#### Keeping Track of Borrows with `RefCell<T>`
- When creating immutable and mutable references, can use the `&` and `&mut` syntax, with `RefCell<T>`, can use the `borrow` and `borrow_mut` methods which are part of the safe API that belongs to `RefCell<T>`
- `borrow` method returns the smart pointer type `Ref<T>` and `borrow_mut` returns the smart pointer type `RefMut<T>`, both types implement `Deref`, can treat them like regular references
- `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are currently active, each time `borrow` is called, the `RefCell<T>` increases its count of how many immutable borrows are active, when a `Ref<T>` value goes out of scope, the count of immutable borrows goes down by 1
   - Just like compile-time borrowing rules, `RefCell<T>` allows many immutable borrows or one mutable borrow at any point in time
   - Trying to violate these rules would result in a panic at runtime
- Choosing to catch errors at runtime rather than compile time means potentially finding mistakes in code later in the development process, also code would incur small runtime performance penalty as a result of keeping track of the borrows at runtime rather than compile time, however, using `RefCell<T>` makes it possible to write a mock object that can modify itself to keep track of the messages it has seen while using it in a context where only immutable references values are allowed, can use `RefCell<T>` despite its trade-offs to get more functionalty than regular references provide

### Allowing Multiple Owners of Mutable Data with `Rc<T>` and `RefCell<T>`
- A common way to use `RefCell<T>` is in combination with `Rc<T>`
- `Rc<T>` alows for multiple owners of some data but only gives immutable access to that data
- Given an `Rc<T>` that holds a `RefCell<T>`a, can get a value that can have multiple owners and can mutate
- For example, in cons example using `Rc<T>` to allow multiple lists to share ownership of another list, since `Rc<T>` holds only immutable values, can't change any of the values in the list once they've been created, can add in `RefCell<T>` for its ability to change the values in the lists
- Example: ```
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    
    *value.borrow_mut() += 10;
}```
- Can create a value that is an instance of `Rc<RefCell<i32>>` and store it in a variable named `value` so it can be directly accessed later, can then create a `List` in `a` with a `Cons` variant that holds `value`, need to clone `value` so both `a` and `value` have ownership of the inner value rather than transferring ownership from value to `a` or having `a` borrow from `value`
- Can wrap this list `a` in an `Rc<T>` so that when creating lists `b` and `c`, they can both refer to `a`
- After creaitng the lists in `a`, `b`, and `c`, to add 10 to the value in `value`, can do this by calling the `borrow_mut` method on `value` which uses automatic dereferencing to dereference the `Rc<T>` to the inner `RefCell<T>` value, the `borrow_mut` method returns a `RefMut<T>` smart pointer and the dereference operator on it to change the inner value
- By using `RefCell<T>`, have an outwardly immutable `List` value but can use the methods on `RefCell<T>` that provide access to its interior mutability to modify the data when necessary
- The runtime checks of the borrowing rules protect from data races and it's sometimes worth trading some speed for this flexibility in data structures
   - Note that `RefCell<T>` does not work for multithreaded code
   - `Mutex<T>` is the thread-safe version of `RefCell<T>`

## Reference Cycles Can Leak Memory
- Rust's memory safety guarantees make it difficult but not impossible to accidentally create memory that is never cleaned up (memory leaks)
- Preventing memory leaks entirely is not one of Rust's guarantees, meaning memory leaks are memeory safe in Rust
- Rust allows memory leaks using `Rc<T>` and `RefCell<T>`, it's possible to create references where items refer to each other in a cycle, this creates memory leaks because the reference count of each item in the cycle will never reach 0 and the values will never be dropped

### Creating a Reference Cycle
- Example: ```
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}```
- This is an example of a reference cycle, another variation of the `List` definition, the second element in the `Cons` variant is now `RefCell<Rc<List>>`, meaning that instead of having the ability to modify the `i32` value, this enables modification of the `List` value a `Cons` variant is pointing to
   - This also adds a `tail` method to make it convenient to access the second term given a `Cons` variant
- Example: ```
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    let b = Rc::new(Cons(5, RefCell::new(Rc::clone(&a))));

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&a);
    }

    //println!("{:?}", a.tail());
}```
- This adds a `main` function that uses the previous definitions, creates a list in `a` and a list in `b`, then it modifies the list in `a` to point to `b` creating a reference cycle
- This also creates an `Rc<List>` holding a `List` value in the variable `a` with an initial list of `5, Nil`, then creating an `Rc<List>` instance holding another `List` value in the variable be containing `5` and pointing to the list in `a`
- `a` is modified so it points to `b` instead of `Nil`, creating a cycle, done by using the `tail` method to get a reference to the `RefCell<Rc<List>>` to change the value inside from an `Rc<List>` that holds a `Nil` value to the `Rc<List>` in `b`
- The reference count of the `Rc<List>` instances in both `a` and `b` is 2 after changing the list in `a` to point to `b`, at the end of `main`, Rust drops the variable `b`, which decreases the reference count of the `b` `Rc<List>` from 2 to 1 as well, the memory that `Rc<List>` has on the heap won't be dropped at this point because its reference count is 1 not 0, then Rust drops `a`, which decreases the reference count of the `a` `Rc<List>` from 2 to 1 as well, the instance's memory can't be dropped either, because the other `Rc<List>` instance still refers to it, the memory allocated to the list will remain uncollected forever
- If there was a memory cycle in a more complex program this may allocate lots of memory in a cycle and use more memory than it needed and might overwhelm the system, causing it to run out of available memory
- When having `RefCell<T>` values that contain `Rc<T>` values or similar nested combination of types with interior mutability and reference counting, must ensure that there are no reference cycles created, can't rely on Rust to catch them
- Creating a reference cycle would be a logic bug in a program that should use automated tests, code reviews, and other practices to minimize
- Another solution for avoiding reference cycles is reorganizing data structures so that some references express ownership and some references don't, as a result, can have cycles made up of some ownership relationships and some non-ownership relationships, and only the ownership relationships affect whether or not a value can be dropped
   - Always want `Cons` variants to own their list, reorganizing the data structure isn't possible

### Preventing Reference Cycles Using `Weak<T>`
- `Rc::clone` increases the `strong_count` of an `Rc<T>` instance and an `Rc<T>` instance is only cleaned up if its `strong_count` is 0
- Can also create a weak reference to the value within an `Rc<T>` instance by calling `Rc::downgrade` and passing a reference to the `Rc<T>`
- Strong references are how to share ownership of an `Rc<T>` instance, weak references don't express an ownership relationship, and their count doesn't affect when an `Rc<T>` is cleaned up, they won't cause a reference cycle because any cycle involving some weak references will be broken once the strong reference count of values involved is 0
- When calling `Rc::downgrade`, this results in a smart pointer of type `Weak<T>`, instead of increasing the `strong_count` in the `Rc<T>` instance by 1, the `Rc<T>` type uses `weak_count` to keep track of how many `Weak<T>` references exist, similar to `strong_count`, the difference is that `weak_count` doesn't need to be 0 for the `Rc<T>` instance to be cleaned up
- Since the value that `Weak<T>` references might have been dropped, to do anything with the value that a `Weak<T>` is pointing to, must ensure the value still exists, can do this by calling the `upgrade` method on a `Weaek<T>` instance, which will return an `Option<Rc<T>>`, will get a result of `Some` if the `Rc<T>` value has not been dropped yet and a result of `None` if the `Rc<T>` value has been dropped, since `upgrade` returns an `Option<Rc<T>>`, Rust will ensure that the `Some` case and the `None` case are handled and there won't be an invalid pointer

#### Crearting a Tree Data Structure: A Node with Child Nodes
- Example: ```
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}```
- To have a `Node` own its children and share that ownership with variables so each `Node` can be accessed in the tree directly, can define the `Vec<T>` items to be values of type `Rc<Node>`, can also modify which nodes are children of another node with `RefCell<T>` in children around the `Vec<Rc<Node>>`
- Example: ```
fn main() {
    let leaf = Rc::new(Node {
        value: 2,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

}```
- This uses stuct definition and creates one `Node` instance named `leaf` with the value `2` and no children and another instance named `branch` with the value `5` and `leaf` as one of its children
- The `Rc<Node>` in `leaf` is cloned and stored in `branch` meaning the `Node` in `leaf` now has two owners, can get from `branch` to `leaf` through `branch.children`, but there's no way to get from `leaf` to `branch`, the reason is that `leaf` has no reference to `branch` and doesn't know they're related, want `leaf` to know that `branch` is its parent

#### Adding a Reference from Child to its Parent
- To make the child node aware of its parent, need to add a `parent` field to `Node` struct definition, the trouble is in deciding what the type of `parent` should be, know it can't contain an `Rc<T>` because that would create a reference cycle with `leaf.parent` pointing to `branch` and `branch.children` pointing to `leaf`, which would case their `strong_count` values to never be 0
- A parent node should own its children: if a parent node is droped, its child nodes should be dropped as well, however, a child should not own its parent, dropping a child node, the parent should still exist, this is a case for weak references, instead of `Rc<T>`, can make the type of `parent` use `Weak<T>`, specifically a `RefCell<Weak<Node>>`
- Example: ```
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}```
- A node will be able to refer to its parent node but doesn't own its parent
- Example: ```
fn main() {
    let leaf = Rc::new(Node {
        value: 2,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
}```
- Creating the `leaf` node starts without a parent, so this creates a new and empty `Weak<Node>` reference instance, at this point, when trying to get the reference to the parent of `leaf` by using the `upgrade` method, this results in a `None` value
- When creating the `branch` node, it will also have a new `Weak<Node>` reference in the `parent` field since `branch` doesn't have a parent node, stil have `leaf` as one of the children of `branch`, once the `Node` instance is inside `branch`, can modify `leaf` to give it a `Weak<Node>` reference to its parent, can use the `borrow_mut` method in `RefCell<Weak<Node>>` in the `parent` field of `leaf` and then use the `Rc::downgrade` function to create a `Weak<Node>` reference to `branch` from the `Rc<Node>` in `branch`, now `leaf` can access its parent

#### Visualizing Changes to `strong_count` and `weak_count`
- All of the logic that manages the counts and the value dropping is build into `Rc<T>` and `Weak<T>` and their implementations of the `Drop` trait, by specifying the relationship from a child to its parent should be a `Weak<T>` reference in the definition of `Node`, this allows parent nodes to point to child nodes and vice versa without creating a reference cycle and memory leaks

## Summary
- `Box<T>` type has a known size and points to data allocated on the heap
- `Rc<T>` type keeps track of the number of references t o data on the heap so that data can have multiple owners
- `RefCell<T>` type with its interior mutability gives a type that can be used when needing an immutable type but need to change an inner value of that type, it also enforces the borroing rules at runtime instead of at compile time
- The `Deref` and `Drop` traits enable a lot of the functionality of smart pointers
- Reference cycles can cause memory leaks and can be prevented using `Weak<T>`
