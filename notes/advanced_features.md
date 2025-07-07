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
- Compiler ensures references are always valid, unsafe Rust has two new types called raw pointers that are similar to references, as with references, raw pointers can be immutable or mutable and are written as `*const T` and `*mut T` respectively, the asterisk isn't the dereference operator, it's part of the type name, in teh context of raw pointers immutable means that the pointer can't be directly assigned after being dereferenced
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
- Have created raw pointers by using teh raw borrow operators: `&raw const num` creates a `*const i32` immutable raw pointer and `&raw mut num` creates `*mut i32` mutable raw pointerHave created raw pointers by using teh raw borrow operators: `&raw const num` creates a `*const i32` immutable raw pointer and `&raw mut num` creates `*mut i32` mutable raw pointer, since these were created directly from a local variable, these particular raw pointers are valid but can't make that assumption about any raw pointer
