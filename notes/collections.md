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
- Can use the `get` method with the index passed as an argument, receive an `Option<&T>` that can be used with `match`
- Rust provides two ways to reference an element so programmeer can choose how the program behaves when there is an attempt to use an index value outside the range of existing elements
- For example, when using an out of bounds index value, the program panics with `[]`, but with `get`, it will return `None`, the first is best used when program should crash on out of bounds value
- When `get` method is passed an index that is outside the vector it returns `None` without panicking, should use this if out of bounds index could happen occaisonally or normally, code will then hav elogic to handle having either `Some(&element)` or `None`
- When program has a valid reference, the borrow checker enforces the ownership and borrowing rules to ensure this reference and any other references to the contents of the vector remain valid
   - Rule states that you can't have mutable and immutable references in the same scope 
   - Applies to this, taking an immutable reference then trying to push an element to it
   - This error is due to the way vectors work: because vectors put the valuex next to each other in memory, adding a new eleent onto the end of the vector might require allocating new memoruy and copying the old elements to new space, if there isn't enough room to put all the elements next to each other where the vector is currently stored
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
- Strings are implemented as a collection of bytes, some methods provide useful functionality when those bytes are interpreted as text
- Creating, updating, reading `String`s, `String` differences from other collections, how indexing into a `String` is complicated by the differences of the interpretation of `String` data

### What is a `String`?
- Rust has only one string type in the core language which is the string slice `str` that is usually seen in borrowed form `&str`
- String slices are references to some UTF-8 encoded string data stored elsewhere
- String literals, for example, are stored in the program's binary and are therefore string slices
- The `String` type (provided by Rust's standard library rather than coded into the core language) is a growable, mutable, owned, UTF-8 encoded string type

### Creating a New String
- Many of same operations are available with `Vec<T>` are available with `String` as well because `String` is implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities
- A function that works the same way with `Vec<T>` and `String` is the `new` function to create an instance: `let mut s = String::new();`
   - This creates a new, emtpy string called `s`, into which the data can be loaded, otherwise will have some initial data to start the string
   - For this use the `to_string` method, which is available on any type that implements the `Display` trait, as string literals do: `let s1 = "initial contents".to_string();`
- Can also use function `String::from` to create a `String` from a string literal: `let s = String::from("initial contents"):`
- Because strings are used for so many things, can use many different generic APIs for strings, providing a lot of options, despite seemed redundancy, all have their place
- `String::from` and `to_string` do the same thing, selection is a matter of style and readability
- Since strings are UTF-8 encoded, can include any properly encoded data in them 

### Updating a `String`
- A `String` can grow in size and contents can change just like the contents of a `Vec<T>`, if more data is pushed into it
- Can use `+` operator or `format!` macro to concatenate `String values`

#### Appending to a String with `push_str` and `push`
- Can grow a `String` by using the `push_Str` method to append a string slice: `s.push_str("more");`
- `push_str` takes a string slice in order to not take ownership of the parameter: `s1.push_str("more");`
- `push` method takes a single character as a parameter and adds it to the `String` `s1.push('a');`

#### Concatenation with the `+` Operator or the `format!` Macro
- One way to combine two existing `String`s is with the `+` operator: ```
       let s1 = String::from("hi");
       let s2 = String::from("w");

       // Will combine s1 and s2 into s3
       let s3 = s1 + &s2;```
- `s3` will cotain both strings, `s1` is no longer valid after the addition, a reference to `s2` was used because of signature of `+` operator that uses the `add` method that's argument is itself and a reference to another string slice
- In standard library, `add` will be defined using generics and associated types, but there are concrete types, which is what happens when calling this method with `String` values
- `+` signature of `add`: `fn add(self, s: &str) -> String {`
   - Adding a reference of the second string to the first string, this is because of the `s` parameter in the `add` function, can only add a `&str` to a `String`
   - Compiler can coerce the `&String` into `&str`
   - When calling the `add` mehtod, Rust uses a deref coercion which turns `&s2` into `&s2[..]`
   - Since `add` does not take ownership of the `s` parameter, `s2` will still be a valid `String` after this operation
   - `add` takes ownership of `self` because self lacks an `&`
   - `add` takes ownership of the first string, appends a copy of the contents of `s2` and then returns ownership of the result
- To concatenate multiple strings, `+` operator becomes difficult, needing to pass a literal and a reference in sequence
   - Can instead use the `format!` macro: `let s = format!("{s1}-{s2}");`, working like `println!` but instead of printing the output to the screen, returning a `String` with the contents
   - Uses references so that this call doesn't take ownership of any of its parameters

### Indexing into `String`s
- Cannto easily access parts of a `String` in Rust by index

#### Internal Representation
- A `String` is a wrapper over a `Vec<u8>`
- An index into a string's bytes will not always correlate to a valid Unicode scalar value, it is the number of bytes that it takes to encode a string in UTF-8
- Rust does not compile code like `&"hi"[0]` to prevent causing bugs that would result from indexing the first byte and returning the byte, not the character there

#### Bytes and Scalar Values and Grapheme Clusters
- There are actually three relevant ways to look at strings from Rust's perspective: as bytes, scalar valyes, and grapheme clusers (closest things to letters)
   - `[224, 164, 224]`, can also be looked at as Unicodec scalar vaues, which is what Rust's `char` type is, then grapheme clusters, which are a collection of symbols that represent an individual character
- Another reason Rust does not allow direct `String` indexing is that it could not be constant time and require an entire `String` walk to find the index

### Slicing Strings
- Indexing into string is often a bad idea because it is not clear what hte return type of the string-indexing operation should be: a byte value, character, grapheme cluster, or string slice 
- If using indices to create slice, Rust requires programmer to be more specific
   - Example: `let s = &s[0..4];`
   - This has potential to panic at runtime the same way as if an invalid index were accessed in a vector

### Methods for iterating Over Strings
- Best way to operate on pieces of strings is to be explicit about whether to use chracters or bytes
- For individual Unicode scalar values, use the `chars` method
   - This will return different characters for certian Unicode values
- Alternatively, the `bytes` method returns each raw byte
- `for c in "abc".chars() {}` and `for b in "abc".bytes() {}`
- Note that valid Unicode scalar values may be made upi of more than one byte
- Getting grapheme clusters from strings is not functionality provided by the standard library

### Strings Are Not So Simple
- Rust has chosen to make the correct handling of `String` data the defauilt behavior for all Rust programs, means programmers have to put more thoguht into handline UTF-8 data up front
- Standard library offers a lot of functionality built of the `String` and `&str` types to help handle these complex situations correctly
- Use methods like `contains` for searching in a string and `replace` for substituting parts of a string with another string

## Storing Keys with Associated Values in Hash Maps
- The `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V` using a hashing function, which determines how it places these keys into memory
- Useful for when looking up data not y using index, but by key that can be of any type

### Creating a New Hash Map
- One way to create an empty hash map is to use `new` and add elements with `insert`
- Example: ```
       let mut scores = HashMap::new();
       scores.insert(String::from("hi"), 10);```
- Need to first `use` the `HashMap` from collections portion of standard library, not automatically brought into sopce in the prelude
- Hash maps store their data on the heap, this `HashMap` has keys of type `String` and values of type `i32`, hash maps are homogenous, all of keys must have same type and all of values must have the same type

### Accessing Values in a Hash Map
- Can get a value out of a hash map by providing its key to the `get` method
- Example: `let h = scores.get(String::from("hi")).copied().unwrap_or(0);`
- `get` returns an `Option<&V>`, if there's no value for that key in the hash map, get will return `None`
- Option is handled here by calling `copied` to get an `Option<i32>` instead of an `Option<i32>` then `unwrap_or` to set `score` to zeri if `scores` does not have an entry for the key
- Can iterate over each key-value pair in a hash map in a similar manner as done with vectors using a `for` loop: ```
                for (key, value) in &scores {
                    println!("{key}, {value}");
                }```

### Hash Maps and Ownership
- For types that imeplement the `Copy` trait, like `i32`, the values are copied into the hash map
- For owned values like `String`, the values will not be moved and the hash map will be the owner of those values
- If inserting references to values in the hash map, the values won't be moved into the hash map
- The values that references point to must be valid for at least as long as the hash map is valid

### Updating a Hash Map
- Each unique key can only have only value associated with it at a time, not vice versa 
- When changing data in a hash map, have to decide how to handle the case when a key already has a value assigned, could kep the old value and ignore the new value, only adding the new value if the key doesn't already have a value, or could combine the old value and the new value

#### Overwriting a Value
- If a key and a value are inserted into a hash map and then there is another insert with the same key but a different value, the value associated with the key will be replaced
- Example: ```
    scores.insert(String::from("hi"), 1);
    scores.insert(String::from("hi"), 2);```

### Adding a Key and Value Only if a Key Ins't Preset
- Hash maps have a API for this called `entry` that takes the key to check as a parameter
- The return value of `entry` method is an enum called `Entry` that represents a value that might or might not exist
- Example `scores.entry(String::from("hi")).or_insert(3);`
- The `or_insert` method on `Entry` is defined to return a mutable reference to the value for the corresponding `Entry` key if that key exists, if not, it inserts the parameter as the new value for this key and returns a mutable reference to the value

### Updating a Value Based on the Old Value
- Another common use for hash maps is to look up a key's value and then update it baed on the old value
- Example ```
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count *= 1;
    }```
- `split_whitespace` method returns an iterator over subslices, separated by whitespace of the value in `text`
- `or_insert` method returns a mutable reference `&mut V` to the value for the specified key
- The mutable reference is stored in the `count` variable 
- In order to assign to that value, must first dereference `count` using `*`, mutable referencegoes out of scope at the end of the `for` loop

### Hashing Functions
- By default, `HashMap` uses a hashing function called SipHash that can provide resistance to DoS attackcs involving hash tables
- Not the fastest hashing algorithm available but trade-off for security
- Can switch to another function by specifying a different hasher
   - A hasher is a type that implements the `BuildHasher` trait, there are libraries that provide hashers implementing many common hashing algorithms

## Summary
- Vectors, strings, and hash maps provide a large amount of functionality necessasry in programs to store, access, and modify data
