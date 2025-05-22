# Ownership & Borrowing Review

### Stack vs Heap Memory
- When attempting to create variables with primitize types such as integers or characters, these are allocated on the stack
- Stack data is fast to access and follows a LIFO structure
- Fixed length data (size known at compile time) is pushed to the stack
- Non-fixed length data, such as `String`s and vectors, that can grow or shrink in length during runtime are allocated on the heap
- Heap memory takes longer to allocate because it must search for a block of memory large enough to fit the requirements of the allocation and mark it in use
- Pushing to the stack is faster than allocating to the heap because its allocation process is much simpler and you need to follow a pointer to get to the heap data
- Main features of ownership are to keep track of what code is using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused addresses, main purpose of ownership is to manage heap data

### Ownership
#### Ownership Rules
- Each value in Rust has an owner
- A value can only have one owner at time
- At the end of its owner's scope, its value will be dropped

### `String` Type
- For non-primitives of a unknown size such as `String`, data is stored on heap, but how deso Rust know when to clean this up?
- Rust calls `drop`, specifically when author of `String` can put the code to return the memory, or goes out of scope
- Drop is implemented on heap allocated types like `String` 
- Values implementing the `Drop` trait cannot implement `Copy` because they cannot have bitwise copies which could copy the destructuor
- Can use `Clone` to create a copy

### References and Borrowing
- In order to use a value without taking ownership, Rust allows a feature called borrowing in which a value can be borrowed by taking a reference
- References are similar to pointers, address that can be followed to access the data stored at that address, owned by some other variable
- Guaranteed to point to a valid value of a prticular type for the life of the reference, take reference using `&T` and dereference using `*`, Rust supports automatic referencing and dereferencing
- Can modify a borrowed value by passing a mutable reference `&mut T`
- There cannot be a mutable reference to something during the life of an immutable reference to prevent data races, can have multiple immutable references
- A reference starts from where it is introduced and continues through the last time that reference is used
- References cannot have overlapping scope, creating mutable references while immutable references exist because that can result in data races
- The compiler can tell when a reference is no longer being used a point before the end of the scope
- At any given time, you can have either one mutable reference or any number of immutable references
- References must always be valid

### Dangling References
- Dangling pointers are pointers that reference a location in memory that may have been given to someone else by freeing some memory while preserving a pointer to that memory
- Rust compiler guarantees no dangling references
   - Compiler ensures data deos not go out of scope before the reference to the data does
   - At end of scope, local variables are deallocated and you cannot return a reference to a value that goes out of ascope and is deallocated, because that would create a dangling pointer
- Dangling pointers are pointers that reference a location in memory that has been freed
