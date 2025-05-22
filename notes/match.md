# Match

### The `match` Control Flow Construct
- Allows value comparison against a series of patterns and then execute code base on which pattern matches
- Patterns can be made up of literal values, variable names, wildcards, and many more
- First pattern that matches the value falls into the associated code block to be used during execution
- `match` keyword followed by an expression, in this case is coin value, `match` can evaluate to any type
- `match` uses arms, arms have two parts: a pattern and some code
   - Example: `Coin::Quarter => 25`
   - Value is `Coin::Quarter` and then `=>` operator separates the apttern and the code to run, in this case is `25`, each arm is separated from the next with a comma

- Compares the resultant value against the pattern of each arm, in order, if a pattern matches the valuye, code associated with the pattern is executed, if pattern doesn't match, execution continues to the next arm, can have as many arms as needed
- Code associated with each arm is an expression, resultant value of the expression in the matching arm is the value that gets returned for the entire match expression
- Can use curly braces to wrap the resulting expression if it is multiline
   - Example: ```
            Coin::Penny => {
               println!("penny");
               1`
            }```

### Patterns That Bind to Values
- Another feature  of match arms is that they can bind to the parts of the values that match the pattern
   - This is how values can be extracted from enum variants
- Can add a variable called `state` to the pattern that matches values of the variant `Coin::Quarter`
- When a `Coin::Quarter` matches, the state variable will bind to the value of that quarter's state, then `state` becomes useable in in the code for that arm

### Matching with `Option<T>`
- Getting inner value `T` out of the `Some` case when using `Option<T>`
- `Option<T>` can be handled using `match` as done with the `Coin` enum
- Can have a branch matching `None` and another matching `Some(i)` where `i` binds to the value contained in `Some` so `i` takes the value of whatever is wrapped in the `Option<T>`
- `match` against an enum, binding varaibles to get data inside, then executing code based on it is ubiquitous in Rust

### Matches are Exhaustive
- `match` arms' patterns must cover all possibilities
- Must exhaust every last possibility in order for the code to be valid, especially in the case of `Option<T>`, when Rust prevents from forgetting to explicitly handle the `None` case, it protects from assuming that there is a value that might have null

### Catch-All Patterns and the `_` Placeholder
- Can take a default action for other values 
- Example: `other => c(other)`
   - Pattern is variable chosen to be named `other`
   - Code runs for the `other` arm, using the variable by passing it
- Catch-all pattern meets requirement that `match` must be exhaustive
- Catch-all arm must go last because other arms must be evaulated in order such that the catch-all appears last
- `_` is a special pattern that matches any value and does not bind to that value, not warned about unused variable
- In order to have nothing happen, replace expression with `()` for the catch-all or default
- Example: `_ => ()`
   - Indicates to Rust that with any value that doesn't match a pattern in an earlier arm, don't run any code in this case
- Can reduce verbosity of `match` expressions with `if let` syntax
