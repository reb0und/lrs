# Control Flow
The ability to run some code depending on whether a condition is `true` and repeatedly when a condition is `true`, appear in `if` statements and loops

### `if` Expressions
- `if` expressions allow code to branch depending on conditions
- ```let number = 3
     if number < 5 {
        println!("number is less than 5");
     } else {
        println!("number is >= 5");
     }```
- Condition must be `bool`
- Can handle multiple conditions with `else if`
- Overusing `else if` can clutter code, use `match`

### Using `if` in a `let` statement
- Conditionals can also be used in let statements
- `let a = if true { 1 } else { 2 };`
- The results of the conditional must be the same type because Rust must know at compile time what type the variable is, letting the compiler verify the type is valid everywhere, this isn't possible if the type is determined at runtime

### Repetition with Loops
- Rust provides several loops: `loop`, `while`, `for`
- `loop` keyword tells Rust to run code until you specify when to stop
- Can break out of code using `break` keyword
- `continue` will complete the current iteration and move on to the next

### Returning Values from Loops
- Can add the value to return after the `break` expression used to stop the loop
- `break 1`;
- Can `return` from within a loop; `break` will exit current loop, `return` will exit current function

### Loop Labels
- If there exist multiple loops, `break` and `continue` apply to the innermost loop at that point
- Loop lables must begin with a single quote `'<loop_name>: loop { break '<loop_name>;};`

### Conditional Loops with `while`
- Loops through code while a condition is true
- `while true {}`

### Looping Through a Collection with `for`
- Example: ```
   let arr: [u8; 3] = [1, 2, 3];
   for element in arr {
       println!("{element}");
   }```
- Can use a `Range` to iterate over a set of numbers, generates all the numbers in a sequence, starting from one number and ending before another number
   - Can also use `rev` to reverse the range
