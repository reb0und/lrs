# Generic Types, Traits, and Lifetimes
- Generics are astract stand-ins for concrete types or other properties
- Generics' behavior can be expressed without knowing how they relate to other generics without knowing what will be in their place during compilation and code execution
- Functions can take generic parameters instead of concrete types like `i32` and `String`, in the same way they take parameteres with unknown values to run the same code on multiple concrete values
   - Some generics are `Option<T>` or `Vec<T>` or `HashMap<K, V>` or `Result<T, E>`
- Can extract functions to reduce code duplication
- Can use traits to define behavior in a generic way, can combine traits with generic types to constrain a generic type to accept only those types that have a particular behavior, instead of just any type
- Lifetimes are a variety of generics that give the compiler information about how references relate to each other
   - Lifetimes give the compiler enough information about borrowed values such that it can ensure references will be valid in more situations than it would without help

### Removing Duplication by Extracting a Function
- Generics allow replacement of specific types with a placeholder that represents multuple types to remove code duplication
- By looking at how to recognize duplicated code to extract into a function, can start to recognize duplicated code that can use generics
- Here, can extract code that finds largest number into a function named `largest`, `largest` has a parameter called `list` which represents any concrete slice of `i32` values might be passed into the function
- Steps for this
   - Identify duplicate code
   - Extract duplicate code into the body of the function and speciy the inputs and return values of that code in the function signature
   - Update the two instances of duplicated code to call the function instead
- How to generalize this?

## Generic Data Types
- Generics are used to create definitions for items like function signatures or structs, which can then use with different concrete data types

### In Function Definitions
- When defining a function that uses generics, place the generics in the signature of the function that would usually specify the data types of the parameters and return value, doing so makes code more flixble and provides more functionality to callers of function while preventing code duplication
- To parametrize types into a new single function, need to name type parameter, as done with value parameters to a function, can use `T` since by convention, type parameter names in Rust are short
- When using a parameter in the body of a function, must declare the parameter in the signature so compiler knowns what the name means, must do the same for a type parameter name in a function signature, must place type name declarations inside angle brackets `<>`, between the name of the function and the parameter list
- Example: `fn largest<T>(list: &[T]): &T {`, read as function `largest` is generic over some type `T`, has one parameteter `list` which is a slice of values of type `T`, and returns a reference to type `T`, won't compile yet, compiler notes to use `std::cmp::PartialOrd` trait, error states that body of `largest` wont work for all possible types that `T` could be, can only use types that can be ordered in order to compare them, ino rder to fix, would need to use help text's suggestions and restrict types valid for `T` to only those that implement `PartialOrd`

### In Struct Definitions
- Can also define structs with generic type parameters in one or more fields using the `<>` syntax
- Example:
      ```struct Point<T> {
          x: T,
          y: T,
      }```
   - Declare name of type parameter inside angle brackets just after the name of the struct, then use the generic type in the struct definition, where concrete data types would otherwise be specified
   - This requires that both `x` and `y` have the same type because they are both defined over type `T`, if an instance of `Point` with different types is made, code won't compile, when the integer value `5` is assigned to `x`, compiler knows `T` is an integer for this instance of `Point<T>`, then when specifying something like `4.0` for `y`, defined this to have the same type as `x`, will get type mismatch
   - To define a `Point` struct where `x` and `y` are both generics but have different types, can use multiple generic types
   - Example: ```
         struct Point<T, U> {
             x: T,
             y: U,
         }```
- Can use as many generic type parameters as possible but overuse can make code difficult to read, lots of generic types may indicate that code needs restructuring into smaller pieces

### In Enum Definitions
- Can define enums to hold generic data types in their variants
- Example: 
         ```enum Option<T> {
             Some(T),
             None
         }``
- This is a generic over type `T` and has two variants: `Some`, which holds one value of type `T` and a `None` variant that doesn't hold any value
- By using `Option<T>` enum, can express the abstract concept of an optional value and since `Option<T>` is generic, can use this abstraction no matter what the type of the optional value is
- Enums can use multiple generic types as well, the definition of the `Result` enum is another example: ```
            enum Result<T, E> {
                Ok(T),
                Err(E),
            }```
- `Result` enum is generic over two types `T` and `E` and has two variants `Ok` which is a value of type `T` and `Err` which is a value of type `E`, this definition makes it convenient to use the `Result` enum anywhere with an operation that might succeed (return a value of some type `T`) or fail (return an error of some type `E`)
- When recognizing situations in code with multipkle struct or enum definitions that differ only in the types of the values they hold, can avoid duplication by using generic types isntead


### In Method Definitions
- Can implement mehtods on structs and enums and use generic types in their definitions too
- Example: ```
         impl<T> Point<T> {
             fn x(&self) -> &T {
                 &self.x
             }
         }```
- Must declare `T` just after `impl` so that can use `T` to specify that methods are implemented on type `Point<T>`, by declaring type `T` as a generic type after `impl`, Rust can identify that type in angle brackets in `Point` is a generic type rather than a concrete type
- Could have chosed different name for the generic parameter rather than the generic parameter declared in the struct definition, but using the same name is conventional
- If writing a method within `impl` that declares a generic type, method will be defined on any instance of the type, no matter what concrete type ends up substituting for the generic type
- Can also specify constraints on generic types when defining methods on the type, for example, implement methods only on `Point<f32>` rather than `Point<T>` instances with any generic type
- Example: ```
         impl Point<f32> {J
             fn distance_from_origin(&self) -> f32 {
                 (self.x.powi(2) + self.y.powi(2)).sqrt()
             }
         }```
- This example means that instances of `Point<T>` where `T` is not of type `f32` will not have this method defined
- Generic type parameters in a struct definition aren't always the same as those used in the struct's method signatures
- Example: ```
         impl<X1, Y1> Point<X1, Y1> {
             fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y1> {
                 Point {
                     x: self.x,
                     y: other.y,
                 }
             }
         }```
- Method signature uses generic types `X1` and `Y1` for the `Point` struct and `X2 Y2` FOR THE `mixup` method signature to make the example clearer
   - Example creates a new `Point` instance with the `x` value from `self` (of type `X2`) and the `y` value from `other` (of type `Y2`)
- Generic parameters `X1` and `Y1` are declared after `impl` because they go with the struct definition, `X2` and `Y2` are declared after `fn mixup` because they're only relevant to the method

### Performance of Code Using Generics
- There is no runtime cost using generics, Rust accomplishes this by monomorphization of the code using generics at compile time
- Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled, in the process the compiler does the opposite of the steps used to create the generic ufnction, the compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with
- During compilation, compiler performs monomorphization and the compiler reads the values that have been used in, for example, `Option<T>` instances and identifies the two kinds of `Option<T>`, `i32` and `f64` and expands the generic definition of `Option<T>` into two definitions specialized to `i32` and `f64`, replacing the generic definition with the specified ones
- The generic `Option<T>` is repalced with the specific definitions created by the compiler, since Rust compiles generic code into code that specifies the type in each instance, no runtime cost is paid for using generics, when code runs, it performs just as it would if it had duplicated each definition by hand
