# Common Collections
- Rust's standard library includes a number of useful data structures called collections
- Most other data types represent one specific value, but collections can contain multiple values
- Unlike built-in array and tuple types, the data these collections point to is stored on the heap, meaning the amount of data does not need to be known at compile time and can grow or shrink as the program runs
- Each kind of collection has different capabilities, costs
- Three collections that are used very often in Rust
   - A vector allows storage of a variable number of values next to each other
   - A string is a collection of characters
   - A hash map allows the association of a value with a specific key, it's a particular implementation of the more general data structure called a map

## Storing Lists of Values with Vectors
- First collection type is vector or `Vec<T>`
- Allows storage of more than one value in a single data structure htat puts all the values next to each other in memory
- Vectors can only store calues of the same type
- Useful when there is a list of items, such as the lines in a text file or prices of items in a shopping cart

### Creating a New Vector
- To create a new empty vector, call the `Vec::new` function
- Example: `let v: Vec<i32> = Vec::new();`
   - A type annotation is added here since no values are initiailly added to the vector and Rust needs to know which values to store
   - Vectors are implemented using generics
   - `Vec<T>` can hold any type and when creating a vector of a specific type, can specify the type within angle brackets
   - Indicating `i32` in this case
- More often, will create a `Vec<T>` with initial values and Rust will infer the type of value to store, rarely need to do this type annotation
- Rust provides the `vec!` macro, which will create a new vector that holds the values it is provided
   - Example: `let v = vec![1, 2 3];`, the integer type is `i32` because that is the default integer type
   - Because initial `i32` values have been provided, Rust ca infer the type of `v` is `Vec<i32>` and the type annotation isn't necessary

### Updating a Vector
- To add elements to a vector, use `push` method
- Example: `v.push(6);`
- Must make it mutable using `mut`, numbers inside are of type `i32` and Rust infers this from the data, no need for `Vec<i32>` annotation

### Reading Elements of Vectors
- There are two ways to reference a value stored in a vector: indexing or using the `get` method
- Use the index value of `2` to get the third element because vectors are indexed by number, sgtarting at zero
- Using `&` and `[]` provides a reference to the element at the indes value
- Can use the `get` method with teh index passed as an argument, receive an `Option<&T>` that can be used with `match`
- Rust provides two ways to reference an element so programmeer can choose how the program behaves when there is an attempt to use an index value outside the range of existing elements
- For example, when using an out of bounds index value, the program panics with `[]`, but with `get`, it will return `None`, the first is best used when program should crash on out of bounds value
- When `get` method is passed an index that is outside the vector it returns `None` without panicking, should use this if out of bounds index could happen occaisonally or normally, code will then hav elogic to handle having either `Some(&element)` or `None`
- When program has a valid reference, the borrow checker enforces the ownership and borrowing rules to ensure this reference and any other references to the contents of the vector remain valid
   - Rule states that you can't have mutable and immutable references in the same scope 
   - Applies to this, taking an immutable reference then trying to push an element to it
   - This error is due to the way vectors work: because vectors put the valuex next to each other in memory, adding a new eleent onto the end of the vector might require allocating new memoruy and copying the old elements to new space, if there isn't enough room to put all the elements next to each other where teh vector is currently stored
   - In this case, the reference would point to deallocated memory, and borrowing prevents this from happening

### Iterating Over the Values in a Vector
- To access each element in a vector in turn, iterate through all the elements rather than use indices to access one at a time
- Can use a `for` loop to get immutable references to each element in a vectlr of `i32` values
- Example: ```for i in &v2 {
                  println!("{i}");
              }```
- Example: ```
    for i in &mut v2 {
        *i += 1;
    }```
   - Iterate over mutable references to each element in a mutable vector in order to make changes to the elements
- To change the vector that the reference refers to, must use the `*` dereference operator to get the value in `i` before modifying anything
- Iterating over a vector, whether immutably or mutably, is safe because of the borrow checker's rules
- If attempt to insert or remove items in the `for` loop bodies, would get a compiler error
- Reference to the vector that the `for` loop holds prevent simultaneous modification of the whole vector

### Using an Enum to Store Multiple Types
- Vectors can only store values that are of the same type which can be inconvenient
- Variants of an enum are defined under the same enum type, when one type is needed to represent elements of different types, can define and use an enum
- Example: ```
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("hi")),
        SpreadsheetCell::Float(1.1),
    ];```
- Can define an enum whose variants will hold the different value types, all the enum variants will be considered the same type, that of the enum
- Then can create a vector to hold that enum, and so, hold different types
- Rust needs to know what types will be in the vector at compile time so it knows how much memory on the heap will be needed to store each element, must also be explicit about which types are allowed in a vector
- If Rust allowed vectors to hold any types, there would be a chance that one or more of the tyupes would cause errors with the operators performed on the elements of that vector
- Using an enum plus a `match` expression means that Rust will ensure at compile time that every possible case is handled
- If the exhuastive set of types a program will get at runtime to store in a vector, the enum technique won't work, instead can use a trait object
- Many useful methods defined on `Vec<T>` by standard library such as `push`, `pop`, that removes the last element

### Dropping a Vector Drops Its Elements
- Similar to `struct`s, a vector is freed when it goes out of scope
- When the vector gets dropped, all of its content is also dropped, meaning the ingegers will be cleaned up
- Borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid

## Storing UTF-8 Encoded Text with `String`s
