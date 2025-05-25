# Traits: Defining Shared Behavior
- A trait defines the functionality a particular type can share with other types
- Traits can be used to define shared behavior in an abstract way
- Trait bounds can be used to specify that a generic type can be any type that has certain behavior
- Traits are similar to interfaces with some differences

### Defining a Trait
- A type's behavior consists of the methods that can be called on that type, different types share the same behavior if the same methods can be called on those types
- Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose
- For example: a `Summary` trait in a media aggregator library crate that can display summaries of data on a `NewsArticle` or `SocialPost` instance, need a summary from each type, can do this by calling a `summarize` method on an instance
- Example: ```
   pub trait Summary {
       fn summarize(&self) -> String;
   }```
- Trait is declared using the `trait` keyword and then the trait's name, can then declare the `trait` as `pub` so that crates depending on this crate can make use of this trait too, inside curly brackets, declare the method signatures that describe the behaviors of the types that implement this trait in this case is `fn summarize(&self) -> String`, after method signature, instead of providing an implementation within curly brackets, use a semicolon. Each type implementing this trait must provide its own custom behavior for the body of the method. The compiler will enforce that any type that has the `Summary` trait will have the method `summarize` defined with this signature exactly
- A trait can have multiple methods in its body: the method signatures are listed one per line and each ends in a semicolon

### Implementing a Trait on a Type
- After defining the desired signatures of the `Summary` trait's methods, can implement it on types in media aggregator
- Example: ```
      impl Summary for NewsArticle {
          fn summarize(&self) -> String {
              format!("{} {} {}", self.headline, self.author, self.content)
          }
      }```
- Implementing a trait on a type is similar to implementing regular methods, the difference is that after `impl`, the name of the trait to implement and then the `for` keyword and then specifying the name of the type to implement the trait for
- Within `impl` block, place method signatures that the trait definition has defined and instead of semicolon after each signature, use curly brackets and fill in the method body with the specific behavior that the methods of the trait to have for the particular type
- Must bring the trait into scope as well as the types in order to use them
- Other crates that depend on this crate can bring the `Summary` trait into scope to implement `Summary` on their on types
- One restriction to note is that can implement a trait on a type only if either the trait or the type or both are local to the crate
   - For example, can implement standard library traits like `Display` on a custom type like `SocialPost` as part of `aggregator` crate functionality because the type `SocialPost` is local to `aggregator` crate, can also implement `Summary` on `Vec<T>` in `aggregator` crate since trait `Summary` is local to `aggregator` crate
   - Cannot implement external traits on external types, cannot implement `Display` on `Vec<T>` within `aggregator` crate since `Display` and `Vec<T>` are both defined in the standard libraryand aren't local to `aggregator` crate, this restriction is called coherence, or the orphan rule, named this because the parent type is not present, ensures other people's code can't break local code and vice versa, without this rule two crates could implement the same trait for the same type and Rust would not know which implementation to use

### Default Implementations
- Sometimes it's useful to have default behavior for one or all the methods in a trait instead of requiring implementations for all methods on every type, then as implementing the trait on a particular type, can keep or override each method's default behavior
- Example: ```
            pub trait Summary {
                fn summarize(&self) -> String {
                    String::from("...")
                }
            }```
- To use a default implementation to summarize instances of `NewsArticle`, specify an empty `impl` block with `impl Summary for NewsArticle {}`
- Creating a default implementation doesn't require changing anything about the implementation of `Summary` on other types since the syntax for ovveriding a default impementation is the same as the syntax for implementing a trait that doesn't have a default implementation
- Default implementations can call other methods in the same trait, even if those other methods don't have a default implementation
- After implementing `summarize_author`, `Summary` trait gives the behavior of the `summarize` method without requiring any more code written

### Traits as Parameters
- Can use traits to define functions that accept many different types
- Example ```
      pub fn notify(item: &impl Summary) {
          println!("news");
      }```
- Instead of a concrete type for the `item` parameter, can specify `impl` keyword and the trait name, the parameter accesses any type that implements the specified trait, in the body of `notify`, can call methods on `item` that come from the `Summary` trait, such as `summarize`, can call `notify` on any instance that implements `Summary`, code that calls the function with any other type like `String` or `i32` won't compile because those types don't implement `Summary`

### Trait Bound Syntax
- The `impm Trait` is sytax sugar for a longer form known as a trait bound that looks like: 
      ```pub fn notify<T: Summary>(item: &T) {
          println!("news");
      }```
- This is equivalent to the example in the previous section but more verbose, place trait bounds with teh declaration of the generic type parameter after a colon inside angle brackets
- the `impl Trait` syntax is convenient and makes for more concise code in simple cases, while the fuller trait bound syntax can express more complexity in other cases
- Using `impl Trait` is appropriate if wanting function to allow `item1` and `item2` to have different types as long as both impement `Summary`
   - Example: `pub fn notify(item1: &impl Summary, item2: &impl Summary) {`
- To force both parameters to have the same type, must use a trait bound: `pub fn notify<T: Summary>(item1: &T, item2: &T) {`
- The generic type `T` specified as the type of the `item1` and `item2` parameters constrains the function such tha the concrete type of the value passes as an argument for `item1` and `item2` must be the same

#### Specifying Multiple Trait Bounds with the `+` Syntax
- Can also specify more than one trait bound, for example enforcing that `item` must implement both `Display` and `Summary` with `+` syntax
   - Example: `pub fn notify(item: &(impl Summary + Display)) {`
   - This is also valid with trait bounds on generic types: `pub fn notify<T: Summary + Display>(item: &T) {`

#### Clearer Trait Bounds with `where` Clauses
- Using too many trait bounds has its downsides, each generic has its own trait boudns so functions with multiple generic type parameters can contain lots of trait bound information between the function's name and parameter list, making the function signature difficult to read, for this reason Rust has an alternate syntax for specifying trait bounds inside a where clause after the function signature
- Example: ```
      fn some_function<T, U>(t: &T, u: &U) -> i32 
      where
          T: Display + Clone,
          U: Clone + Debug,
      {}```
### Returning Types That Implement Traits
- Can also use the `impl Trait` syntax in the return position to return a value of some type that implements a trait
- Example: `fn returns_summarizable() -> impl Summary {}`, can specify that `returns_summarizable` implements the `Summary` trait without naming the concrete type
- The ability to specify a return type only by the trait it impements is especially useful in the context of closures and iterators which create types that only the compiler knows or types that are very long to specify, the `impl Trait` allows for specification that a function returns a type that implements the `Iterator` trait without needing to write out a very long type
- Can only use `impl Trait` if returning a single type, code that returns either one or the other would not work

### Using Trait Bounds to Conditionally Implement Methods
- By using a trait bound with an `impl` block that uses generic type parameters, can implement methods conditionally for types that implement the specified traits, for example, the type `Pair<T>` always implements the `new` function to return a new instance of `Pair<T>` (`Self` is a type alias for the type of the `impl` block, which in this case is `Pair<T>`), in next `impl` block, `Pair<T>` only implements the `cmp_display` method if its inner type `T` also implements the `PartialOrd` trait that enables comparison and the `Display` trait that enables printing
- Can also conditioanlly impmenet a trait for any type that implements another trait, implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are used extensively in the Rust standard library, an example is the standard library implements the `ToString` trait on any type that implements the `Display` trait
- Example: `impl<T: Display> ToString for T {}`, since standard library has blanket implementation, can call the `to_string` method defined by the `ToString` trait on any type that implements the `Display` trait, for example, can turn integers into their corresponding `String` values because integers implement `Display`
- Blanket implementations appear in the documentation for the trait in the "Implementors" section
- Traits and trait bounds let code exist that uses generic type parameters to reduce code duplication and also specify to the compiler that the generic types should have particular behavior, compiler can then use the trait bound information to check that all the concrete types used with code provide the correct behavior, Rust moves the errors of calling methods aren't implemented by a type to compile time so programmer must fix this before code even runs, additionally, don't need to write code to checks for behavior at runtime since checks have occurred at compile time, doing so improvies performance without having to give up flexibility of generics
