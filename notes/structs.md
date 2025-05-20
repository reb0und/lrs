# Structs
A struct is a data type that lets you group together and name multiple related values that make up a meaningful group, similar to an objects data attributes 

## Defining and Instantiating Structs
- Similar to tuples, both hold multiple related values
- Pieces of structs can be different types
- Each piece of data can be named to make it clear what each value means, adding names makes structs more flexible than tuples, no need to rely on order of the data to specify or access values of an instance
- Example: ```
   struct user{
      active: bool,
      username: string,
      sign_in_count: u64,
   }
   ```
- To define a struct, enter `struct` keyword and name the struct, name should describe the significance of the pieces of data being grouped together
- Inside curly braces, define names and types of pieces data, called fields
- To use a struct after defining it, create an instance of the struct by specifying concrete values for each of the fields
- Create an instance by stating the name of the struct and then add curly brackets containing `key: value` pairs
   - Do not need to specify the fields in the same order in whcih we declare them in the struct
- To get specific value from a struct, use dot notation, `user1.email`
- If instance is mutable, can change value by using the dot notation and assigning into a particular field
   - Entire instance must be mutable, Rust does not allow specific fields to be marked as mutable
   - Can always construct a new instance of a a struct as the last expression in a function body in order to implicitly return that new instance

### Using the Field Init Shorthand
- Since parameter names and struct field names are same, can use field init shorthand to behave the same but not include the repition of `username` and `email` but just including `username` and `email` since both params have the same name, only need to write `email` rather than `email: email`

### Creating Instances from Other Instances with Struct Update Syntax
- In order to create a new instance of a struct that includes most of the values from another instance, but alter some, use struct auto update syntax
- ```
    let user3 = User {
        email: String::from("another"),
        ..user1
    }```
- Pass `..<other_struct_name>` at the end of the struct instantiation, remaining fields not explicitly set should have the same values as the fields in the given instance
   - Can specify as many values you want, order does not matter
- Struct update syntax uses `=` like an assignment because it moves the data, can no longer use `user` after creating `user2` because the `String` in the username of the field `user1` was moved into `user2`, if had given `user2` new `String` values for both `email` and `username`, `user1` would still be valid after creating `user2`, since `active` and `sign_in_count` implement the `Copy` trait, since stack data only can be copied, can also still use `user1.email` since its value was not moved out

### Using Tuple Structs Wihout Named Fields to Create Different Types
- Rust also supports structs that look similar to tuples, called tuple structs
- Tuple structs have added meaning the struct name provides but don't have names associated with their fields, just have types of the fields
- Useful for when you want to name a tuple, to distinguish a tuple from other tuples, and when naming each field in a regular struct would be verbose or redundant
- Example definition: `struct Point(i32, i32, i32);`
- Example instantiation: ```
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);```
    - `black` and `origin` values are different because they're instances of different tuple structs
    - Function taking `Color` as an parameter cannot take `Point` as an argument, despite having the same composition
    - Similar to tuples in that they can be destructured into their individiual pieces and can use a `.` followed by the index to access an individual value
    - Tuple structs require you to name the type of the struct when you destructure them
      - `let Point(x, y, z) = point`

### Unit-Like Structs without any Fields
- Can also define structs that don't have any fields
- Called unit-like structs because they behave similarly to `()`, the unit type of a tuple
- Useful to implement a trait on some type but don't have any data to store in the type itself
- Example: `struct AlwaysEqual;`, no need for curly brackets or parentheses

### Onwership of Struct Data
- `User` struct uses the owned `String` type rather than the `&str` string slice type, this means each instance of the struct owns all of its data and the data is valid as long as the entire struct is valid
- Alos possible for structs to store references to data owned by something else, requires use of lifetimes
- Lifetimes ensure that data referenced by a struct is valid for as long as the struct is, trying to store a reference in a struct without specifying lifetimes

### Adding Useful Functionality with Derived Traits
- Can include functionality to print out debugging info by adding `[#derive(Debug)]` above the struct
   - Have to explicitly opt into this functionality for a struct
   - Can print output using `{<struct_name>:?}`
   - Can prettier print using `{<struct_name>:#?}`, easier to read for larger structs
- `dbg!` macro
   - Another way to print out a value using the `Debug` format is to use the `dbg!` macro, which takes ownership of an expression (as opposed to `println!` which takes a reference), and prints the file and line number of where the `dbg!`  macro call occurs in code long with resultant value of the expression and returns ownership of the value
   - Calling `dbg!` macro prints to standard error console stream `stderr` as opposed to `println!` which prints to standard output console stream `stdout`
- How to refactor code to turn `area` function into an `area` method defined on `Rectangle`

## Method Syntax
- Methods are similar to functions, declared with `fn` keyword and a name, they can have parameters, and a return value, and contain some code that's run when method is called externally
- Unlike functions, methods are defined within the context of a struct, enum, or trait object, and their first parameter is always `self`, which represents the isntance of the struct the method is being called on

### Defining Methods
- Changing `area` function that has a `Rectangle` instance as a prameter and instead make an `area` method defined on the `Rectangle` struct
- To define a function within the context of `Rectangle`, start with an `impl` (implementation block), for `Rectangle`
- Everything within the `impl` block will be associated with the `Rectangle` type
   - Place `area` function within `impl` block, change `rectangle` parameter to `self` and everywhere in body
   - In main, can use method syntax instead, to call `area` on `Rectangle` instance, method syntax goes after an instance; add a dot followed by method name, parentheses, and any arguments
   - In signature for `area` use `&self` sintead of `rectangle: &Rectangle`, which is short for `self: &Self`
   - Within an `impl` block, the type `Self` is an alias for the type that the `impl` block is for, so `Rectangle` in this case
   - Methods must have a parameter named `self` of type `Self` for their first parameter spot, rust allows an abbreviation, with only the name `self` the first parameter spot
   - Still need to use `&` in front of the `self` shorthand to indicate that this method borrows the `Self` instance, just as done in `rectangle: &Rectangle`
   - Methods can take ownership of `self`, borrow self immutably, or borrow `self` mutably, just as they can any other parameter
   - `&self` is chosen here for same reason chose `&Rectangle`, no need to take ownership, just want to read the dat in the struct, not write
   - To change the data in the instance, use `&mut self` as the first parameter
   - Having method that uses `self` as parameter is rare, usually used when you want to prevent caller from using the origin instance after the transformation
   - Main reason for methods instead of functions is for organization
      - Can put everything to do with an instance of a type in one `impl` block, rather than making future users of code search for capabilities of `Rectangle` in various places in provided library
   - Can give a method the same name as one of the struct's fields
      - Example: ```
                   fn width(&self) -> bool {
                       self.width > 0
                   }```
   - Using automatic dereferencing and referencing Rust does not need two different operators to call methods on something like `->` and `.`
   - When you call a method with `object.something()`, Rust automatically adds in `&`, `&mut`, or `*`, so `object` matches the signature of the method
      - The following are the same `p1.distance(&p2);` and `(&p1).distance(&p2);`
   - This automatic referencing works because methods can have a clear receiver
   - Given the receiver and name of a method, Rust can figure out definitively whether the method is reading `&self`, mutating `&mut Self`, or consuming `self`, making borrowing implicit

### Methods with More Parameters
- Example: ```
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }```

### Associated Functions
- All functions defined with an `impl` block are called associated functions because they're associated with the type nmaed after `impl`
- Associated functions that don't have `self` as their first parameter (and are not methods) because they don't need an instance of the type to work with
   - An example of this is the `String::from` function that's defined from the `String` type
- Associated functions that aren't methods are often used for constructs that will return a new instance of the struct(often called `new`, but `new` isn't a special name and isn't built into the language)
- Example: ```
    fn square(size: u32) -> Self {
        Self {
            width: size,
            length: size,
        }
    }```
    - Associated function `square` that has one dimension parameter, used as both width and height, making it easier to create a square `Rectangle`, rather than having to specify the same value twice
    - `Self` keywords in the return type and in the boyd of the function are aliases for the type that appears after the `impl` keyword (`Rectangle`)
      - Can also pass the type directly `Rectangle` instead of `Self`
   - To call this function, the `::` syntax with the struct name is needed
      - Example: `let sq = Rectangle::square(10);`
      - This function is namespaced by the struct, the `::` syntax is used for both associated functions and namespaces created by modules

### Multiple `impl` Blocks
- Each struct is allowed to have multiple `impl` blocks

### Summary
- Structs allow custom types that are meaningful for a domain
- By using structs, associated pieces of data can be kept connected to each other and name each piece to make your code clear
- In `impl` blocks, functions can be defined that are associated with a type, methods are a kind of associated function that let you specify the behvaior that instances of a struct have
