# Error Handling
- Rust has a number of features for handling errors
- Rust requires the acknowledgement of the possibility of an error and taking some action before code compilation
- This requirement makes a program more robust by ensuring error discovery and handlingbefore deploying to production
- Rust groups errors into two major categories: recoverable and unrecoverable errors
   - For a recoverable error such as a file not found error, might want to report the problem and retry the operation
   - Unrecoverable errors are always symptomps of bugs, accessing a location beyond the end of an array, so immediately stop program
- Most languages distinguish between these two kinds of errors and handles both in the same way using mechanisms like exceptions, Rust does not have exceptions
- Rust has the type `Result<T, E>` for recoverable errors and the panic! that stops execution when the program encounters an unrecoverable error

## Unrecoverable Errors with `panic`!
- When bad things happen in code, Rust has the `panic!` macro
- There are two ways to cause a panic in practice: by taking an action that causes code to panic (accessing an array past the end), or by explicitly calling the `panic!` macro
- By default, these panics will print a failture message, unwind, clean up the stack, and quit
- Via an environment variable, can have Rust display the call stack when a panic occurs to amke it easier to track the source of the panic

### Unwinding the Stack or Aborting in Response to a Panic
- By default, when a panic occurs hte program starts unwinding, Rust walks back up the stack and cleans up the data from each function it encounters
- Walking back and cleaning up the stack is a lot of work, Rust allows choice of immediately aborting, which exits the program without cleaning up
- Memory that program was using will then need to be cleaned up by the OS
- To make resultant binary as small as possible, can switch to aborting upon a panic by adding `panic = 'abort'` to the appropriate `[profile]` sections

- Can call `panic!` with `panic!("crash and burn")`
- First line shows where panic occurred, second shows message
- Panic call may be in code that code calls, and filename, line, number may be in external code, local line of code that led to the `panic!` call
- Can use the backtrace of the functions `panic!` came from to figure out what part of code is causing the problem 
- Can use the backtrace of the functions the `panic!` call came from to figure out the part of the code that is causing the problem
- Rust will panic in attempt to access an out of bounds index
- In C, ths could lead to a buffer overread, Rust protects from this by stopping execution
- Can set the `RUST_BACKTRACE` environment variable to get backtrace of error cause
- Each number represents a call
- In order to get call stack, debug symbols must be enabled
- Debug symbols are enabled by default when using `cargo build` or `cargo run` without the `--release` flag
- Current call is in line 6 
- Should start looking at the location pointed to by the first line
- Will need to figyre out what action code is taking with what values to cause the panic and what code should do instead

## Recoverable Errors with `Result`
- Most errors are not serious enough to require existing the program, sometimes when a function fails it is for a reason that can easily be interpreted and responded to
- `Result` enum is defined as having two variants `OK` and `Err`
   - ```enum Result<T, E> {
            Ok(T),
            Err(E),
        }```
   - The `T` and `E` are generic type parameters, `T` represents the type fo the value that will be returned in a success case within the `Ok` variant and `E` represents the type of error that will be returned in failure with the `Err` variant
   - Since `Result` type has generic type parameterrs, can use the `Result` type and the functions defined on it in many different wituations where the success value and error value may differ
- Example: `let f = File::open("hello.txt");`
   - The return type of `File::open` is a `Result<T, E>`, the generic type parameter has been filled in by the implementation of `File::open` with the type of the success value `std::fs:File` which is a file handle
   - The type of `E` used in the error value is `std::io::Error`, this return type menas the call to `File::open` may succeed and return af ile hadnle that can be read from or write to
   - Call my also fail, `File::open` needs way to indicate whether it succeeded or failed at same time it provides the file handle or error info which is what `Result` enum conveys
   - If `File::open` succeeds. value in the variable `f` will be an instance of `Ok` that contains a file handle, when it fails it will be an instance of `Err` that cotains more info about hte kind of error that occurred
   - Example: ```
    let r = match f {
        Ok(file) => file,
        Err(e) => panic!("{e:?}")
    };```
   - `Result` and its variants have been brought into scope by the prelude, no need to specify `Result` before the `Ok` and `Err` variants
   - One arm of match handles retrieving the file, other handles the error by panicking with `panic!`

### Matching on Different Errors
- Code will `panic!` no matter which failure `File::open` has, may want to take different actions depending on failure reasons
- For example, if file does not exist, create it
- Type of value that `File::open` returns inside the `Err` variant is `io::Error` which is a struct provided by the standard library, has a method `kind` to get an `io::ErrorKind` value, enum has variants representing the different kinds of errors that might result from an `io` operation
- Variant to use is `ErrorKind::NotFound`, indicates the file does not exist yet, match on `f` but also inner match on `error.kind()`
- Can create new file on if `error.kind()` is `NotFound` but since `File::create` could also fail, need a second arm in the inner `match` expression

### Alternatives to Using `match` with `Result<T, E>`
- `match` expression is useful but primitive, can eventually use closuers thatcan be more concise than using `match` when handling `Result<T, E>` in code

### Shortcuts for Panic on Error: `unwrap` and `expect`
- Using `match` can be verbose and doesn't always communicate intent well
- `unwrap` is one of many helper methods defined on `Result<T, E>` to do various more specific tasks, `unwrap` is a shortcut method implemented just like the `match` expression
- If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok` but if it is the `Err` variant, it will call the `panic!` macro
- Example: `let f = File::open("abc.txt").unwrap();`
- `expect` method also allows for the selection of the `panic!` error message
- Example: `let f = File::open("abc.txt").expect("error opening file");`
- `expect` is used in the same way as `unwrap` to return the file handle or call the `panic!` macro 
- Error message used by `expect` will be the parameter that is passed to `expect`, rather than the default `panic!` message that `unwrap` uss
- Good to choose `expect` over `unwrap` and give more context about why the operation is expected to always succeed, if assumptions are ever proven wrong, have more information to use in debugging

### Propagating Errors
- When a function's implementation calls something that might fail, insteaf of handling the error within the function itself, can return the error to the calling code so that it can decide what to do, this is known as propagating the error and gives more control to the calling code, where there might be more information or logic that dictates how the error should be handled
- Example ```
   fn read_user_from_file() -> Result<String, io::Error> {
       let user_file_result = File::open("hello.txt");

       let mut username_file = match user_file_result {
           Ok(file) => file,
           Err(e) => return Err(e)
       };

       let mut username = String::new();

       match username_file.read_to_string(&mut username) {
           Ok(_) => Ok(username),
           Err(e) => Err(e),
       }
   }```
- Reads username from a file, if file doesn't exist or can't be read, function will return those errors to the code that called the function
- Function returns a `Result<String, io::Error>`, where `T` is a `String` and `E` is an `io::Error`
- If this code succeeds without any problems the code that clals the function will receive an `Ok` value that holds a `String`, the `username`, calling code will receive an `Err` value that holds an instance of `io::Error`, `io::Error` is chosen as the return type of this function because this is the type of error value that is returned from both of the operations being called in this function's body that might fail:`File::open` and `read_to_string`
- Starts by calling the `File::open` function then handling that result with a `match`, if this succeeds, the file handl ein the pattern variable `file` becomes the value  in the mutable variable `username_file` and the function continues, in the `Err` case, instead of calling `panic!`, the `return` keyword is used to return early out of the function entirely, then a mutable `String` is created as a buffer and the `read_to_string` takes a mutable reference to that `username` `String`, if this succeeds, function succeeds and pass a `Ok(username)` as result, otherwise `Err(e)` with contained error, no need to epxlicitly say `return` since end of function
- Code that calls this will either receive an `Ok` value that contains a username or an `Err` valye that contains an `io::Error`, up to the calling code to decie what to do with those values
- If calling code gets an `Err` it could call `panic!` and crash the program, use a default username, or look up the username from somewhere other than a file
- Here, all the success or error information is propagated upward to be handled appropriately
- The practice of propagating errors is so common in Rust that Rust provides the question mark operator `?` to make this easier

#### A Shortcut for Propagating Errors: the `?` Operator
- Example: `let username_file = File::open("hello.txt")?;`
- The `?` placed after a `Result` value is defined to work in almost the same way as the `match` expressions defined to handle the previous `Result` values
- If the value is an an `Ok`, the value inside the `Ok` will be returned from this expression, if this is an `Err`, the `Err` will be returned from the whole function as if the `return` keyword was used and the error value will get propagated to the calling code
- Difference between the previous `match` expression and what the `?` operator does, error values that come from the `?` operator called on them go through the `from` function, defined in the `From` trait in the standard library, which is used to convert values from one type into another
- When the `?` operator calls the `from` function, the error type received is converted in the error tyep defined in the return thpe of the current function
- Useful when a function returns one error type to represent all the ways a function might fail
- Example would be constructing an instance of `OurError` from an `io::Error`
- `?` eliminates a lot of boilerplate, `?` will return value inside the `Ok` to the variable and if it errors, `?` will return early out of the whole function and give any `Err` value to the calling code
- `fs::read_string` function that opens files and creates a new `String`, reads through the contents of the file, writes the contents to that `String`, and returns it

#### Where The `?` Operator Can Be Used
- The `?` operator can only be used in functions whose return type is compatible with the value `?` is used on, this is because the `?` operator is defined to perform an early return of a value out the function, in the same manner as the `match` expression
- Return type has to be a `Result` so that it's compatible with this return
- Cannot be used in this `main` function since it has the return type of `()`, not `Result`
- Only can use `?` operator in a function that returns `Result`, `Option` or another type that implements `FromResidual`
- To fix error, change return type to be compatible with value using the `?` operator on, other choice is to us a `match` or one of the `Result<T, E>` methods ot handle the `Result<T, E>` in whatever way is appropriate
- `?` can be used with `Option<T>` values as well, can only use `?` on `Option` on a function that returns an `Option`, same behavior as when called on a `Result<T, E>`, if value is `None`, none will be returned early from the function, if the value is `Some`, `Some`'s internal value is the resultant value of the expression
- `?` won't automatically convert a `Result` to an `Option` or vice versa, in those cases, can use methods like the `ok` method on `Result` or the `ok_or` method on `Option` to do the conversion explicitly
- `main` function typically returns `()`, there are restrictions on what its return type can be for the program to behave as expected
- `main` can also return a `Result<(), E>`
- Example: `fn main() -> Result<(), Box<dyn Error>>{`
- The `Box<dyn Error>` type is a trait object, for now, can be read as any kind of error
- Using `?` on a `Result` value in a `main` function with the error type `Box<dyn Error>` is allowed because it allows for any `Err` value to be returned early
- When a `main` function returns a `Result<(), E>`, the executable wil exit with a value of `0` if `main` returns `Ok(())` and exit with a nonzero value if `main` returns an `Err` value
- Executables written in C return integers when they exit, programs that exit successfully return the integer `0`, programs that exit return something other than `0`, Rust also does this
- The `main` function may return any types that implement the `std::process::Termination` trait which contains a function `report` that returns an `ExitCode`

## To `panic!` or Not to `panic!`
- When code panics, theres no way to recover, whether there's a possible way to recover or not, this is making the decision that a situation is unrecoverable on behalf of the calling code
- When choosing to return a `Result`, this is giving the calling code options to attempt to recover that is appropriate for its situation, or it could dercide that an `Err` value in this case is unrecoverable and call `panic!` and turn recoverable error into an unrecoverable one, `Result` is a good default choice when dealing with a function that may fail
- In situations such as examples, prototype code, tests: may be more appropriate to write code that panics instead of returning a `Result`

### Examples, Prototype Code, and Tests
- When dealing with an example to illustrate some concept, including robust error handling can make the example less clear
- `unwrap` and `expect` meethods are useful when prototyping before being ready to decide how to handle errors, leaving markers in code for when making program more robust
- If a method call fails in a test, whole test should fail, since `panic!` is how a test is marked as a failure, calling `unwrap` or `expect` is what should happen

### Cases in Which There Is More Information Than the Compiler
- Would also be appropriate to call `unwrap` or `expect` when given some other lgoic that ensures the `Result` will have an `Ok` value but the logic isn't something the compielr understands
- Will still have a `Result` value to handle, despite the situational logical impossibility
- Should use `expect` to indicate the reason why it should be valid

### Guidelines for Error Handling
- Code shouild panic when it's possible that code could end up in a bad state
   - A bad state is when some assumption, guarantee, contract, or invariant has been broken, such as invalid, contradictory, or missing values are passed to code, plus the following:
      - The bad state is somehting that is unexpected, as opposed to something that would happen occaisonally, like a user entering data in the wrong format
      - Code after this point needs to rely on not being in this state, rather than checking for the problem at every step
      - There's not a good way to encode this information in the types used
- If code is called with values that don't make sense, best to return an error so user of library can decide what to do in that case
- In cases where continuing could be insecure or harmful, best choice might be to call `panic!` and alert the person using library to the bug in their code so they can fix it during development
- `panic!` is often appropriate if calling external code that is out of cointrol and returns an invalid state that there is no way of fixing
- When failure is expected, it's more appropriate to return a `Result` than make a `panic!` call
- Examples include a parser given malformed data or an HTTP request getting rate limited, in these cases a `Result` indicates that failure is an expected possibility that the calling code must decide how to handle
- When code performs an operation that could put user at risk if called using invalid values, code should verify the values are invalid first and panic
- Since functions have contracts, behavior is only guaranteed if the inputs meet particular requirements, panicking when contract is violated makes sense since a contract violation indicates a caller side bug
- Despite this, lots of error checks can be verbose and annoying, Rust's type system and compiler can do many checks
- If function has a particular type as a parameter, can proceed with code's logic knowing that the compiler has already ensured there is a valid value
- For exmaple, if there is a type rather than an `Option`, program expects to have something rather than nothing and code doesn't have to handle two cases for the `Some` and `None` variants, will have one cases for definitely having a value, code trying to pass nothing to a function won't compile

### Creating Custom Types for Validation
- Can create a new type in the dedicated module and put the validations in a function to create an instance of the type rather than repeating the validations everywhere, this way it is safe for functions to use the new type in their signatures and confidently use the values they receive
- Can insert checks for validation in a constructor or `new` associated function

## Summary
- `panic!` macro signals that program is in a state it can't handle and can indicate to the process to stop instead of trying to proceed with problematic values
- `Result` enum uses Rust's type system to indicate that operations might fail in way that code could recover from, can indicate to code that works with a `Result` value need to handle both success and failure
- Using `panic!` and `Result` in appropriate situations make code more reliable
