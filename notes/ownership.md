# Ownership
Enables Rust to make memory safety guarantees without needing a garbage collector. Ownership is a set of rules and state how a Rust program manages memory. Memory is managed through a system of ownership with a set of rules that the compiler checks. If any rules are violated, program will not compile. None of these features degrade performance.

### The Stack and the Heap
- Whether a variable is on the stack or heap affects how language behaves and why you need to make certain decisions. Both the stack and the heap are parts of memory available to your code at runtime but structured differently.
- Stack stores values in the order it gets them and removes the values in the opposite order, LIFO
   - Adding data is pushing onto stack and removing data is popping off stack
- All data stored on the stack must have a known, fixed size, data with an unknown size at compile time or a size that might change must be stored on the heap
- When data is stored on the heap, a certain amount of space is requested (less organized than stack)
- Memory allocator finds an empty spot in the heap that is big enough, marks it in use, and returns a pointer (address of that location)
   - This is called allocating on the heap (pushing values onto the stack is not considered allocating)
- Since the poitner to the heap is a known, fixed size, this pointer can be stored on the stack; data must be stored on the heap, data is retrieved through folllowing the pointer on stack
- Pushing to stack is faster than allocating on the heap because the aloocator never needs to search for a place to store new data, that location is always on the top of the stack
- Allocating on heap is more work because allocator must first find a big enough space to hold the data and then perform operations to prepare for the next allocation
- Accessing heap data is slower than accessing data on stack because you need to follow a pointer to get there, processors are faster if they jump around less in memory, processor can do its job better if it works on data that's close to other data rather than farther away
- When code calls a function, the values passed to the function (including potentially, pointers to the data on the heap) and the function's local variables get pushed onto the stack.
- When the function completes, those values are popped off the stack
- Keeping track of what code is using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unsued addresses are features of ownership, the main purpose of ownership is to manage heap data.

### Ownership Rules
- Each value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

### Variable Scope
- A variable is valid from the point at which it's declared until the end of the current scope
   - An example is: ```{ let s = "hello"; }```
   - When `s` comes into scope, it is valid
   - It remains valid until it goes out of scope

### The `String` Type
- Previous primitives are of a known size, can be stored on the stack, and popped off the stack when their scope is over, and can be quickly and trivially copied to make a new, independent instance if another part of code needs to use the same value in a different scope
- `String` is stored on the heap, how does Rust know when to clean up this data?
- String literals are immutable, not suitable for every situation in which text is used, also because not every string value can be known at compile time
- Rust has a second string type `String`, manages allocated data on the heap, as such is able to store an amount of text that is unknown at compile time
   - Can be created using `let s = String::from("hello");`
   - `::` allows for namespace `from` from under the `String` type
   - Can be mutated

### Memory and Allocation
- String literal contents are known at compile time, text is hardcoded directly into final executables, this only comes from string literal's immutability
- Need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents
   - Memory must be requested from the memory allocator at runtime
   - Need a way of returning this memory to the allocator when done with the `String`
- First part is done when `String::from` is called, it requets the memory it needs
- Second part, in languages without a GC, it typically programmer's responsibility to identify when memory is no longer being used and free it, just as it was created, if this is forgotten, memory is leaked, but if this is done too early, there may be an invalid variable, if this is done twice, another bug, need to pair one `allocate` with exactly one `free`
- In Rust, memory is automatically returned one the variable that owns it goes out of scope
   - Rust calls `drop`, specifically where the author of `String` can put the code to return the memory


### Variables and Data Interacting with Move
- Given this example: ```
    let s1 = String::from("hello"):
    let s2 = s1;```
- `String` is made up of 3 parts: pointer to the memory contents of the string, a length, a capacity all stored on the stack
- The length is how much memory (in bytes) the contents of the `String` are currently using
- The capacity is the total amount of memory (in bytes) that the `String` has received from the allocator
- When `s1` is assigned to `s2`, `String` data is copied, meaning the pointer, length, and capacity that are on stack are copied
   - Data on heap that pointer refers to is not copied
- When `s1` and `s2` go out of scope, they will both try to free the same memory, known as a double free error, can lead to memory corruption -> security vulnerabilities
- To ensure memory safety, after the line `let s2 = s1;`, `s1` is no longer considered valid, Rust doesn't need to free anything when `s1` goes out of scope
- Concept of copying the pointer, capacity, and length, without copying the actual data is may be similar to making a shallow copy, since Rust also invalidates the first variable, instead of a shallow copy this is known as a move
   - `s1` moved into `s2` 
   - With only `s2` valid, when it goes out of scope, it alone will free the memory
   - Rust will never automatically create deep copies of data, any automatic copying is inexpensive

### Scope and Assignment
- Inverse is true for relationship between scoping, ownership, and memory being freed via `drop` as well
- When you assign a new value to an existing bariable, Rust will call `drop` and free original value's memory

### Variables and Data interacting with Clone
- To deeply copy the data of the `String`, not just the stack data, can use function `clone`
   - Example is `<variable_name>.clone()`
- Arbitrary code is executed that may be expensive

### Stack-Only Data: Copy
- Problem:
      ```let x = 5;
         let y = x;

         println!("x = {x}; y = {y}");```
- This code is valid and there is no need to call clone but `x` was not moved into `y`
- This is because integers have a known or fixed size at compile time are stored completely on the stack, copies are quick to make
   - No reason between deep and shallow copying here, calling clone wouldn't do anything different from shallow copying
- Rust has a special annotation called the `Copy` trait that can be plraced on types that are stored on the stack
   - If a type implements the `Copy` trait, variables that use it do not move, but rather are copied, making them still valid after assignment to another variable
   - If a type has implemented the `Drop` trait, it cannot be annotated with `Copy`
      - This is because it would copy a reference to itself from `Drop`, and result in a double-free
   - All scalar primitives and nothing that requires allocation or is some form of resource can implement `Copy`, such as `u32`, `bool`, `f64`, `char`, tuples like `(i32, i32)`; `(i32, String)` does not implement `Copy`

### Ownership and Functions
- Mechanics of passing a value to a function are similar to those when assigning a value to a variable
- Passing a variable to a function will move or copy, just as the assignment does

### Return Values and Scope
- Returning values can also transfer ownership
- Ownership of a variable follows the same pattern every time: assigning a value to another variable moves it
- When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop
