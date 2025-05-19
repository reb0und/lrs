# Variables, Mutability, and Data Types
## Variables and Mutability
- Variables are immutable by default
    - Once a value is bound to name, value cannot be changed
- Immutable variables can be created using the `mut` keyword
    - ```let mut x = 5;```

### String interpolation
Strings can be interpolated as `"{x}"`, where x is a variable

### Assignment
- New variables are assigned via the `let` keyword
    - ```let x = 5;```

### Constants
- Constants are like variables in the sense that they are bound by name and cannot be changed
- Can be delcared in any scope, including global
- Naming convention for constants is to use all uppercase letters and underscores for spaces
- Usage `const SIGNAL: u8 = 5`:

### Shadowing
- New variables can be declared with the same name as a previous variables which is called shadowing, first variable is shadowed by the first, the compiler will see that value when using the name of that variable
- Difference between shadowing and reassigning with `mut` is shadowing basically creates a new variable when reusing `let`
- Can also change the type of the variable, but reuse the name
- Shadowing can spare from needing to create variables with different names

## Data Types
- Every value in Rust is a data type, indicates what kind of data is beign specified
- Rust is statically typed, such that it must know the types of all variables at compile time, this can usually be inferred, but should be specified during conversions
- For example, when converting a `String` to a numeric type with parse
    - ```let guess: u32 = "42".parse();```
- Two initial types of data: scalars and compounds

### Scalar Types
- Represent a single value: integers, floating-point numbers, booleans, and, characters

### Integer Types: numbers without fractional component
- Can be signed or unsigned with `i` or `u` prefix
- Must also include `<n>` where n is the number of bits of space to consume
- Goes from 8-128, also includes arch (size of a pointer on specific architecture)
- Signed variants can store numbers from -(2^(n-1)) to 2^(n-1) - 1 inclusive
    - `i8` range is (-2)^7 to 2^7 or -128 to 127
- Unsigned variants can store numbers from 0 to 2^n - 1
    - `u8` can store numbers from 0 2^8 - 1 to 255
- `isize` and `usize` depend on computer's architecture: 64 bits if on 64 bit architecture 32 bits if on 32 bit architecture

### Integer literals can exist as decimals, hex, octals, binary, or bytes (`u8` only)
- Number literals that can be multiple numeric types allow a type suffix such as `57u8` to designate the type
- Number literals can also use `_` as a visual separator to improve readability (`1_000`)

### Integer Overflows
- Integer overflows occur when a value outside the range of an integer type is attempted to be assigned
- These checks do not occur in release mode compilation
- Compiler will perform two's complement wrapping, value greater than max will wrap around to min
    - If type is `u8` and value is 256 -> 0 and 257 -> 1, without panic, creates unexpected behavior

### Floating-Point types
- Rust has two primitive types for floats: `f32`, `f64` (32 and 64 *more precise* bits respectively)

### Numeric operations
- Rust supports basic mathematical operations: addition, multiplication, division, modulus
        - Integer division truncates toward zerot to the nearest integer

### Booleans
- `bool`, One byte in size, represent `true` or `false`

### Character type
- `char`, most primitive alphabetic type, specified with single quotes, string literals use double quotes
- Four bytes in size, represents a Unicode Scalar Value (more than just ASCII), ranges from `U+000` to `U+D7FF` and `U+E000` tp `U+10FFFF` inclusive, discussed further in Chapter 8

### Compound Types
- Can group multiple types into one type, Rust has two primitive compound types: tuples and arrays

### Tuple type
- Tuples are a way of grouping together a number of values with heterogenic types into a single compound type
- Tuples have a fixed length, zero-indexed
- Defined as `(<T>...)`
- Tuple variable will bind to the entire tuple because a tuple is considered a single compound element
- Tuples are destructured by passing a matching pattern in assignment to the tuple from it as such
    - ```
    let tup: (u8, char) = (1, 'a');
    let (x, y) = tup;```
- Tuple elements can also be accesssed directly using a period `(.)` followed by the index of the value to access
- Tuples without any names have special name: `unit`, value and corresponding type are both `()` and represent an empty value or return type

### Array Type
- Another way to have a collection of multiple values is with an array, but all elements but have homogenic types, and arrays in Rust have a fixed length
- `let a: [i8; 3] = [1, 2, 3]`
    - [<type>; length]
- Useful when you want data to be allocated on stack (similar to previous types) rather than heap (discussed further in Chatper 4)
- Arrays aren't as flexible as vectors (allow mutable arrays that grow/shrink in size), more often should use vectors
- Arrays are useful when number of elements won't need to change (days in week)
- Arrays can be initialized via `let a = [i8; 5];`, type followed by semicolon and size
- Array elements can be accessed using `a[0]`, `<array_name>[i]`
- Will receive runtime error if attempt to retrieve value at out of bounds index
    - Rust prevents invalid memory accesses
