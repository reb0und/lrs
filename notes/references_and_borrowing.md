# References and Borrowing
- A reference is like a pointer, it's an address that can be followed to access the data stored at that address, owned by some other variable
- A reference is guaranteed to point to a valid value of a particular type for the life of that reference
- To take the reference of a variable inclue a `&` before a value
- Define an argument of a reference to a variable as `&T`
- Ampersands represent references and allow you to refer to some value without taking ownership of it
- Opposite of referencing by using `&` is dereferencing, accomplished using `*`
- No need to return values to release ownership since no ownership in the first place
- Creating a reference is called borrowing
- How to modify something being borrowed? 
   - Both variables and references are immutable by default, cannot modify something with just a reference

### Mutable References
- Can modify something being borrowed using mutable references
- Pass or define reference as `&mut <variable_name`
   - `&mut` indicates that the variable is a mutable reference, or that the value it points to is mutable
   - Function signature with a parameter that takes a mutable reference would be `some_string: &mut String`
   - Mutable references have restriction: if there is a mutable reference to a value, there can be no other references to that value, cannot borrow a value as a mutable mopre than once at a time
   - This restriction preventing multiple references to the same data at the same time allows for controlled mutation, limits data races at compile time
   - Data races are similar to race conditions and occur when these happen
      - Two or more pointers access the same data at the same time
      - At least one of the pointers is being used to write the data
      - There is no mechanism to synchronize access to the data
   - Data races cause undefined behavior and are difficult to diagnose and track down at runtime, Rust refuses to compile code with data races
   - Cannot borrow something as a mutable that is also borrowed as immutable
   - A reference starts from where it is introduced and continues throguh the last time that reference is used
   - Cannot have references that overlap scope, basically create mutable references while immutable references exist because that can result in data races
   - Compiler can tell reference is no longer being used a point before the end of the scope

### Dangling References
- Dangling pointers are pointers that reference a location in memory, that may have been given to someone else, by freeing some memory while preserving a pointer to that memory
- In Rust, the compiler guarantees that references will never be dangling references
   - If there is a reference to some data, compiler ensures data does not go out of scope before the reference to the data does
   - Since at end of scope, local variables are deallocated, you cannot return a reference to a value that goes out of scope and is deallocated as that would create a dangling pointer

### The Rules of References
- At any given time, you can have either one mutable reference or any number of immutable references
- References must always be valid
