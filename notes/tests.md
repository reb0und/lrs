# Writing Automated Tests
- Can write tests to assert that when a value is passed to a function, it has a deterministic output (passing `3` to `add_two` results in `5`), can run these tests whenever changes are made to code to ensure any existing correct behavior has not changed

## How to Write Tests
- Tests are Rust functions that verify that the non-test code is functioning in the expected manner, the bodies of test functions typically perform these three actions
   - Set up any needed data or state
   - Run the code to test
   - Assert that the results are expected
- Some features Rust provides for these actions include the `test` attribute, a few macros, and the `should_panic` attribute

### The Anatomy of a Test Function
- At its simplest, a test in Rust is a function that's annotated with the `test` attribute, attributes are etadata about pieces of Rust code
- One example is the `derive` attribute used with structs, to change function into a test function add a `#[test]` on the line before `fn`, when running tests with `cargo test` command, Rust builds a test runner binary that runs the annotated functions and reports on whether each test function passes or fails
- Whenever making a new library project with Cargo, a test module with a test function in it is automatically generated, providing a template for writing tests without needing to look up exact structure and syntax each time starting a project, can add as many test functions and modules as wanted
- Example: ```
      #[cfg(test)]
      mod tests {
          use super::*;

          #[test]
          fn it_works() {
              let result = add(2, 2);
              assert_eq!(result, 4);
          }
      }```
- `#[test]` annotation indicates that this is a test function so test runner knos to treat this function as a test, might also have non-test functions in the `tests` module to help set up common scenarios or perform common operations, always need to indicate which functions are tests
- Example function body uses `assert_eq!` macro to assert that result, which contains the result of calling `add` with 2 and 2 equals 4, this assertion serves as an example of the format for a typical test
- Can test this with `cargo test` which will compile and run the test
- Can also mark a test as ignored so it doesn't run in a particular instance
- Can also pass an argument to the `cargo test` command to run only tests whose name matches a string, called filtering
- Can also have benchmark tests that measure performance, only available in nightly Rust
- Also have `Doc-tests` which are the results of any documentation tests
- Rust can compile code examples that appear in API documentation, helps keep docs and code in sync
- Can make a test fail with `panic!`, each test is run on a new thread, and when the main thread sees that a test thread has died, test is marked as failed, simplest way to panic is to call the `panic!` macr
   - Two sections appear between the individual results and the summary: first displays the detailed reason for each test failure, next section lists the names of all the failing tests, useful when there are lots of tests detailing failing test output

### Checking Results with the `assert!` Macro
- The `assert!` macro, provided by the standard library, is useful when intending to ensure that some condition in a test evaluates to `true`
- Give the `assert!` macro an argument that evlautes to a boolean, if value is `true`, nothing happens and test passes, if value is `false`, the `assert!` macro calls `panic!` to cause the test to fail, using `assert!` macro helps check that code is functioning in intended way
- Example: `assert!(larger.can_hold(&smaller));`
   - `can_hold` method is a perfect use case for the `assert!` macro since it returns a boolean
- The `use super::*` line inside `tests` module brings the code under test in the outer module into the scope of the inner module, use a glob here so anything defined in the outer module is available to this `tests` module
- Have called the `assert!` macro and passed it the result of calling `larger.can_hold(&smaller)`
- Can test the opposite on `smaller.can_hold(&larger)` and negate the result before passing it to the `assert!` macro

### Testing Equaltiy with `assert_eq!` and `assert_ne!` Macros
- A common way to verify functionality is to test for equality between the result of the code under test and the expected value the for the code to return, could do this using `assert!` macro and pasing an expressin using the `==` operator, instead can use `assert_eq!` and `assert_ne!` to do this more conveniently, these macros compare two arguments for equialty or inequality respecivelt and print the two values of the assertion fails, making it easier to see why the test failed
      - Example: ```
           let result = add_two(2);
           assert_eq!(result, 4);```
      - During failure, left and right arguments are provided
- `assert_ne!` macro will pass if the two values are not equal and fail if they're equal, useful for when it is known what a value shouldn't be
- Under the surface, `assert_eq!` and `assert_ne!` macros use the `==` and `!=` operators respectively, when the assertions fail, these macros print their arguments using debug formatting, which means values being compared must implement `PartialEq` and `Debug` traits
- All primitive types and most of standard library types implement these traits
- For structs and enums that are locally defined, need to implement `PartialEq` to assert equality of those types, also need to implement `Debug` to print the values when the assertion fails
- Because both traits are derivable, this is as simple as adding the `#[derive(PartialEq, Debug)]` annotation to struct or enum definition

### Adding Custom Failure Messages
- Can also add cusomt messages to be printed with the failure message as optional arguments to the `assert!`, `assert_eq!`, and `assert_ne!` macros, any arguments specified after the required arguments are passed along to the `format!` macro
- Useful for documenting what an assertion means
- Example: `assert!(true, "{result}");`

### Checking for Panics with `should_panic`
- It's important to check that code handles error conditions as expected, for example, consider the `Guess` type, can write test that ensures that attempting to create a `Guess` isntance with a value outside of that range panics
- Example: ```
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(101);
    }```
- Place the `#[should_panic]` attribute after the `#[test]` attribute before the test function it applies to
- Tests that use `should_panic` can be imprecise, a `should_panic` test would pass even if the test panics for a different reason from the expected one, can add an optional `expected` parameter to `should_panic` attribute, making sure failure message contains the provided text
- Test will pass when value placed in the `should_panic` attribute's expected parameter is a substring of the message that the function panics with
- What is chosen to specify demands on how much of panic message is unique or dynamic and how precise test should be
- Can swap this out, failure message with test indicates that it panicked as expected but panic message did not include the expected string

### Using `Result<T, E>` in Tests
- Can rewrite tests to use `Result<T, E>` and return an `Err` instead of panicking
- Example: ```
          #[test]
          fn it_works() -> Result<(), String> {
              let result = add(2, 2);

              if result == 4 {
                  Ok(())
              } else {
                  Err(String::from("2 + 2 != 4"))
              }
          }```
- This function now has the `Result<(), String>` return type, in the body of the function, rather than calling the `assert_eq!` macro, return `Ok(())`, when test passes and an `Err` with a `String` inside when the test fails
- Writing tests so they return a `Result<T, E>` enables usage of the question mark operator `?` in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an `Err` variant
- Cannot use `#[should_panic]` annotation on tests that use `Result<T, E>`, to assert that an operation returns an `Err` variant, don't ue the question mark operator on the `Result<T, E>` value, just use `assert!(value.is_err())`

## Controlling How Tests Are Run
- `cargo test` compiles code in test mode and runs the resultant test binary, default behavior of the binary produced by `cargo test` is to run all the tests in parallel and capture output generated during test runs, preventing the output from being displayed and making it easier to read the output related to test results, can specify comand line options to change this default behavior
- Some command line options go to `cargo test` and some go to resultant test binary, arguments that go to `cargo test` are listed and followed by the separator `--` and then the ones that go to the test binary
- Running `cargo test --help` displays options that can be used with `cargo test`, and running `cargo test -- --help` displays the options that can be used after the separator

### Running Testrs in Parallel or Consecutively
- 
