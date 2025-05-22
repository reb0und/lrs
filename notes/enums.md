# Enums
- Enums or enumerations allow a type be defined by enumerating its possible variants
- Enum called `Option` expresses that a value can be either something or nothing
- Can pattern match using the `match` expression to make it easy to run different code for different values of an enum
- `if let` is another way to use enums in code

### Defining an Enum
- Enums provide a way of saying a value is one of a posible set of values
   - A `Rectangle` is one of a possible set of shapes including `Circle` and `Triangle`
   - Can define these possibilities as an enum
- IP addresses are a usecase for enums because there are two major standards versions four and six
   - Can enumerate all possible variants
   - Any IP address can be either a version four or version six address but not both at the same time
   - This is appropriate for enums because enums can only be one of its variants but can be treated as the same type when handling different situations that apply to any kind of IP address
- Example: ```
   enum IpAddrKind {
       V4,
       V6
   }```

### Enum Values
- Can create instances of an enum: `let four = IpAddrKind::V4;`
- Define function that takes any `IpAddrKind`: `fn route(ip_kind: IpAddrKind) {}`
- Call the function with any variant `route(IpAddrKind::V4)`
- Can place data directly inside an enum variant by defining each enum variant with a parameter `V4(String)`, providing each variant with an associated `String` value
- Can create an instance of a `V4` address as such `let home = IpAddr::V4(String::from("127.0.0.1"));`
   - Can attach data to each variant of the enum diretly
   - Name of each enum variant becomes a function that constructs an instance of the enum `IPAddr:V4()` is a function call that takes a `String` argument and returns an instance of the `IpAddr` type
   - This constructor function is automatically defined as a result of defining the enum
- Each variant can have different tyupes and amounts of associated data
   - Version four IP addresses will always have four numeric components that will have values between 0 and 255
   - Can express `V4` addresses as four `u8` values but still express `V6` addresses as one `String` value, would not be accomplishable with structs
   - Example: ```
         enum IpAddr {
              V4(u8, u8, u8, u8),
              V6(String),
          }```
- Can put any kind of data inside an enum variant: strings, numeric types, structs, even an enum
- Example: ```
   enum Message {
       Quit,
       Move { x: i32, y: i32 },
       Write(String),
       ChangeColor(i32, i32, i32):
   }```
   - `Quit` has no data associated with it at all
   - `Move` has named fields, similar to a struct
   - `Write` includes a single `String`
   - `ChangeColor` includes three `i32` values
- Using an enum instead of structs to handle all variants makes it easy to define a function to take any kind of these messages with the `Message` enum
- Can define methods on enums using `impl`
   - Example: ```
            impl Message {
                fn call(&self) {}
            }```
   - `self` would be used to get the value that was called from the method
   - `let m = Message::Write(String::from("hi"));` is what `self` will be in the body of the `call` when `m.call()` runs

### The `Option` Enum and Its Advantags Over Null Values
- `Option` type encodes the scenario in which a value could be something or it could be nothing
- For example, requesting first value in an arbitrary list could be nothing
- Compiler can check whether all the cases that should be handled have been handled
- Rust does not have null, a value that means there is no value
- If a null value is attemped to be used as a non-null value
- Will receive an error
- Rust has an enum that can encode the concept of a vlaue being present or absent called `Option<T>`
   - Defined as follows: ```
      enum Option<T> {
         None,
         Some(T),
      }```
   - `<T>` is a generic type parameter, `<T>` means that the `Some` variant of the `Option` enum can hold one piece of data of any type
- Rust can infer assignemnts of `Some` because a value has been placed inside the `Some` variant 
- Rust requires an annotation for `None` because the compiler can't infer the type corresponding to the `Some` variant with only a `None` value
   - `let absent_number: Option<i32> = None;`
- Given a `Some` value, there is a value present within the `Some`
- Given a `None` value, it to some extent is the same as null
- This is better than having null because `Option<T>` and `T` are different types and the compiler won;'t allow use an `Option<T>` as a value as if it were definitely a valid value
- This eliminates the risk of incorrectly assuming a not-null value to be not null
- Then, when using that value, must explicitly opt in by making the type of a value `Option<T>`, when using the value, required to explicitly handle case where value is null
   - Everywhere that a value isn't an `Option<T>`, can safely assume that the value isn't null
- How to extract the `T` out of a `Some` variant given a value of type `Option<T>` so that the inner value is useable? `Option<T>` has a large number of methods useful in a variety of situations
- In order to use an `Option<T>` value, should ahve code that will handle each variant, want code that will run only given a `Some(T)` value that is allowed to use the inner `T` value, and some code to run only if there is a `None` value and the code doesn't have a `T` value available
- `match` expression is a control flow construct that does this when used with enums, will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value
