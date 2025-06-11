# Functional Language Features: Iterators and Closures
- Programming in a functional style often includes using functions as values by passing them in arguments, returning them from other functions, assigning them to variables for later execution, and so forth
- Closures are a function-like construct that can be stored in a variable
- Iterators are a way of processing a series of elements

## Closures: Anonymous Functions That Capture Their Environment
- Rust's closures are anonymous functions that can be saved in variables or passed as arguments to other functions
- Closures can be created in one place and then called elsewhere to evaluate it in a different context
- Unlike functions, closures can capture values from the scope in which they're defined

### Capturing the Environment with Closures
- Example: ```
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }```
- In `giveaway` method, the user preference is given as a parameter of type `Option<ShirtColor>` and call the `unwrap_or_else` method on `user_preference`, the `unwrap_or_else` method is defined by the standard library, taking one argument: a closure without any arguments that returns a value `T` (the same type stored in the `Some` variant of the `Option<T>`, in this case `ShirtColor`)
   - If the `Option<T>` is the `Some` variant, `unwrap_or_else` returns the value from within the `Some`, if the `Option<T>` is the `None` variant, `unwrap_or_else` calls the closure and returns the value returned by the closure
- The closure expression `|| self.most_stocked()` is the argument to `unwrap_or_else`, this is a closure that takes no parameters itself (if the closure had parameters, they would appear between the two vertical bars)
   - The body of the closure calls `self.most_stocked()`, the closure is defined there and the implementation of `unwrap_or_else` will evaluate the closure later if the result is needed
- Closure has been passed that calls `self.mosted_stacked()` on the current `Inventory` instance, standard library did not need to know anything about the `Inventory` or `ShirtColor` types defined or the logic used in this scenario, the closure captures an immutable reference to the `self` `Inventory` instance and passes it with the code specified to the `unwrap_or_else` method, functions, on the other hand, are not able to capture the environment in this way

### Closure Type Inference and Annotation
- There are more differences between functions and closures:
   - Closures don't usually require annotation of the types of parameters or the return values like `fn` functions do
   - Type annotations are required on functions because the types are part of an explicit interface exposed to users
   - Defining this interface rigidly is important for ensuring that the types of values a function uses and returns is agreed upon
   - Closures on the other hand, aren't used in an exposed interface like this, they're stored in variables, and used without naming them and exposing them to users of a library
- Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario, within these limited contexts, the compiler can infer the types of the parameters and the return type, similar to how it's able to infer the types of most variables
- As with variables, type annotations can be added to increase explicitness and clarity at the cost of being more verbose than is strictly necessary
- Example: ```
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating...");
        thread::sleep(time::Duration::from_secs(2));
        num
    };```
- With type annotations, the syntax of closures is similar to that of functions
- Example ```
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1;
    fn add_one_v1(x: u32) -> u32 { x + 1 }```
- This example illustrates how closure syntax is similar to function syntax except for the use of pipes and the amount of syntax that is optional
   - First line shows a function definition, second line shows a fully annotated closure definition, third line shows removed type annotations from the closure definition, the fourth line removes brackets that are optional because the closure body has only one expression
   - All are valid definitions that will provide the same behavior when called, `add_one_v3` and `add_one_v4` require the closures to be evaluated to be able to compile because the types will be inferred from their usage, similar to `let v = Vec::new();` needing either type annotations or values of some type to be inserted into the `Vec` for Rust to be able to infer the type
- For closure definitions, the compiler will infer one concrete type for each of their parameters and their return value
- Example: ```
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);```
- The first time `example_closure` is called with the `String` value, the compiler infers the type of `x` and the return type of the closure to be `String`, these types are then locked into the closure in `example_closure` and a type error is received when attempting to use a different type with the same closure

### Capturing References of Moving Ownership
