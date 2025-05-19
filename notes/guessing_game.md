# Guessing Game

### Steps
1. Create random number
2. Take input
3. Test input against random num
4. Ouput result

#### Dependencies
- Import dependencies using `use`
   - An example is `use std::io`
   - Need `io` to obtain user input and print result as ouput, originates from the standard library or `std`
- By default, every Rust program has a set of items from the standard library it brings into the scope of every program called the prelude
   - If you want to use code not in prelude, need to import it via `use`

#### Reading Guess
1. Allocate buffer using mutable string: `String::new()`
   - The `::` indicates that the `new()` is an associated function of the `String` type, creates a new instance of a `String`
      - An associated function is a function that is implemented on a type
      - New creates a new empty string, found on a lot of types
2. Read line using stdin: ```io::stdin().read_line(&mut guess).expect("error")``` 
   - Pass a mutable reference to the guess/buffer, `&` indicates that a reference is being passed, `&mut guess`
      - A way to let multiple parts of code access one piece of data without needing to copy that data into memory multiple times
         - References are immutable by default, need to write `&mut guess` instead of `&guess` to make it mutable
   - This is mutable because the program must be able to write and modify the data
   - `read_line` puts whatever user enters into string passed to it but returns a `Result` value
      - `Result` is an enum, can be in one of multiple possible states
      - Each state is a variant
         - `Result`'s variants are `Ok` and `Err`
            - `Ok` indicates operation was successful and it contains the successfully generated value
            - `Err` means operation failed and contains info about the failure
            - Values of `Result` have methods defined on them, such as `expect`
               - If the instance of `Result` is an `Err` value, `expect` crashes program and displays the message passed as an argument to expect
               - If the `Result` is an `Ok` value, `expect` will take the return value that `Ok` holds and return it 
               - If you don't call expect, you will get warning
   - Failure is handled using `.expect()`

#### Printing values with `println!` placeholders
- Can print the result of an expression using empty brackets as such: `println!("{}", 1 + 1);`, empty brackets serve as placeholders
- Can also have multiple values: `println!("{}, {}", 1, 2);`

#### Generating a number
- `rand` crate, library crate (contians code that is intended to be used in other programs, cannot be executed alone)
- Add dependency to `Cargo.toml` using `rand = "<version>"`, and building with `cargo build`
- Can also add with `cargo add rand`
- Cargo fetches latest versions of dependencies from registry which in thiscase is crates.io
- Cargo checks the `[dependencies]` section and adds any crates that aren't already downloaded
- Cargo creates a Cargo.lock file to specify versions of dependencies, used for figuring out versions 
- Can update crate to new version with `cargo update`
- Use `rand::Rng`
   - `Rng` method defines methods random number generators implement
   - Trait must be in scope to use those methods
   - Range is specified in the `gen_range` function, takes a range expression as an argument and generates a random number in the range 
      - `start..=end`, inclusive of lower and upper bounds
   - Running `cargo doc --open` will build documentation provided by dependencies locally and open in browser
- Generate random value with ```let num = rand::thread_rng().gen_range(1..=10);```

#### Comparing guess to number
- Using `std::cmp::Ordering`, which is an enum that has the variants `Less`, `Greater`, and `Equal`, which are the 3 outcomes possible when comparing two values
   - Can compare two values, takes reference to values
- Uses a match expression to handle the events, made up of arms, each arm represents pattern to match against and the code that should run if the value given to `match` fits the arm's pattern
   - Rust takes the value given to it and looks through each arm's pattern
   - Can allow you to handle a variety of situations
- Must compare two values of numeric types
   - Convert string to integer with `guess.trim().parse().expect("error");`
      - Trims whitespace, parses value, and handles error
      - Parse function converts a string to another type, this type is indicated in the variable definition when `let guess: u8` is specified with `u8`

#### Handling invalid input
- Instead of crashing on bad input, make user reinput guess
- Can handle Enum results with `Ok(a)` where a is the result
- If `parse` successfully returns number, `Ok` value will contain resultant number and `Ok` value will match the first arm's pattern and return the value, otherwise it will return an `Err` value that contains more information about the error
