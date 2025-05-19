# Functions
- `main` is Entrypoint to a Rust program
- Function can be declared with `fn`
- Rust convention is snake case for function and variable names
- Function is defined using `fn` followed by function name and a set of parentheses, and curly brackets indicating where function body begins and ends
- Can call any function by entering its name followed by a set of parentheses
- Order of functions does not matter

### Parameters
- `another_function(5)` called with concrete values as such
- `another_function(x: i8)` types specified as such
- Parameter types must be declared in function signatures, this way params don't need to be respecified

### Statements and Expressions
- Function values are made of up a series of statements optionally ending in an expression, functions themselves are also statements
- Statements: instruction that perform some action and do not return a value
    - `let x = 5;` is an example of a statement
    - Do not return values, cannot assign something to them, nothing for variable to bind to
- Expressions: evaluate to a resultant value
    - `5 + 6` is an example of an expression, evaluates to `11`
    - In the statement `let x = 6;`, `6` is an expression that evaluates to `6`
    - Calling a function or macro is an expression, so is a scope block with curly brackets
    - ```{
            let x = 3;
            x + 1
         }``` evaluates to 4, last line does not include a semicolon because it is an expression (expressions do not semicolons), adding a semicolon at the end would turn it into a statement and not return a value
### Functions with Return Values
- Return values aren't named but are declared with their type after an arrow `->`
- Return value of a function is synonymous with the value of the final expression in the block of the body of a function
- Can use the return value of a function to initialize a value
- Can just leave end of function as expression or include `return`
