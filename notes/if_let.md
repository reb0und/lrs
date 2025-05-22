# Concise Control Flow with `if let` and `let else`
- The `if let` syntax allows for the combination of `if` and `let` into a less verbose way to handle values that match one pattern while ignoring the rest
- `if let` syntax takes a pattern and an expression separated by an equal sign, works the same way as a match, where the expression is given to the `match` and the pattern is its first arm
- Example: ```
          if let Some(max) = config_max {
              println!("{max}");
          }```
   - In this case, the pattern is `Some(max)` and `max` binds to the value inside the `Some`, `max` is then used in the body of the `if let` block in the same way it was used in the corresponding `match` arm
   - The code in the `if let` block only runs if the value matches the pattern
- Using `if let` means less typing, indentation, and boilerplate code, but losing the exhaustive checking that `match` enforces
   - Need to choose whether gaining conciseness is trade-off for losing exhaustive checking
- Can include `else` with `if let`, block of code that goes with `else` is thge same as the block of code that would go with the `_` case in the `match` expression that is equivalent to the `if let` and `else`
- Example counting coins, announcing quarters' states and computing total non-quarters: ```
         let mut count = 0;
         if let Coin::Quarter(state) = coin {
            println!("{state:?}");
         } else {
            count += 1;
         }```

### `let...else`
- A common expression is to perform some computation when a value is present and return a default value otherwise
- Rust has `let...else` which takes a pattern on the left side and expression on the right, similar to `if let`, but does not have an `if` branch, only an `else` branch, if the pattern matches, it will bind the value from the pattern in the outer scope, if the pattern does not match, the program will flow into the `else` arm, which must return from the function

### Summary
- Covered how to use enums to create custom types that can be one of a set of enumerated values
- Shown how to use `Option<T>` type to prevent errors
- When enum values have data inside them, can use `match` or `if let` to extract and use those values, depending on how many cases to handle
