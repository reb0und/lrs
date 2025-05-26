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
- By default, tests run in parallel using threads, must make sure tests do not depend on each other or on any shared state, including a shared environment such as current workding directory or environment variables
- If tests should not run in parallel or tests should have a limited number of threads, can use the `--test-threads` flag and the number of threads to use to test the binary
   - Example: `cargo test -- --test-threads=1`
   - Here, number of threads is set to `1`, telling the program not to use any parallelism, running tests using one thread will take longer than runnign them in parlalel, but tests won't interfere with each other if they share state

### Showing Function Output
- By default, Rust's testing library captures anything printed to standard output. If calling `println!` in a function and test passes, won't see `println!` output in the terminal, will only see line indicating test passed, if test fails, will see whatever was printed to standard output with error message
- To see printed values for passing tests as well, can tell Rust to show the output of successful tests with `cargo test -- --show-output` flag

### Running a Subset of Tests by Name
- Since running a full test can take a long time, can run only the tests pertaining to code in a particular area, can choose which tests to run by passing `cargo test` the name or names of the tests to run as an argument

#### Running Single Tests
- Can run single tests with `cargo run <test_name>` to run only that test
- Test output will indicate that more tests did not run by displaying the amount of filtered out tests at the end, can specify the names of multiple tests in this way, only the fist values given to `cargo test` will be used

#### Filtering to Run Multiple Tests
- Can specify part of a test name and any test whose name matches that value will be run
- Example: `cargo test can`, runs all tests with `can` in their names
- note that module in which a test appears becomes part of the test's name, can run all tests in a module by filtering the module's name

### Ignoring Some Tests Unless Specifically Requested
- Sometimes a few specific tests can be very time-consuming to execute, may want to exclude them during runs of `cargo test`
- To do this, can annotate time-consuming tests with `ignore` attribute to exclude them `#[ignore]`
- To run ignored tests use `cargo test -- --ignored`
- By controlling which tests run, can make sure `cargo test` results will be returned quickly, when at a good point where it makes sense to check the results of `ignored tests`, can run `cargo test -- --ignored` instead, to run all tests whether they're ignored or not, can run `cargo test -- --include-ignored`

## Test Organization
- Rust community thinks of tests in terms of two main categories: unit tests and integration tests
   - Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces
   - Integration tests are entirely external to library and use code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test 

### Unit Tests
- Purpose of unit tests is to test each unit of code in isolation from the test of the code to quickly pinpoint where code isn't working as expected, unit tests will go in src directory in each file with code that is tested, convention is to create a module named `tests` in each file to contain the test functions and to annotate the module with `cfg(test)`

#### The Tests Module and `#[cfg(test)]`
- The `#[cfg(test)]` annotation on the `tests` module tells Rust to compile and run the test code only when running `cargo test`, not running `cargo build`, saving compile time when only want to build the library and saves space in the resultant compiled artifact because the tests are not included
- Since integration tests go in a different directory, they don't need the `#[cfg(test)]` annotation, since unit tests go in the same files as the code, can use `#[cfg(test)]` annotation to specify that they shouldn't be included in the compiled result
- Attribute `cfg` stands for configuratio, indicates to `Rust` that the following item should only be included given a certain configuration option
- In this case, the configuration option is `test`, provided by Rust for compiling and running tests, by using `cfg` attribute, Cargo compiles test code only if actively running the tests with `cargo test`, including any helper functions that might be within this module, in addition to the functions annotated with `#[test]`

#### Testing Private Functions
- Rust's privacy rules allow testing of private functions by placing the private code in the ancestor or parent module and testing in child module

### Integration Tests
- In Rust, integration tests are entirely external to a library, they use the library in the same way any other code would, which means they only call functions that are part of library's public API, their purpose is to test whether many parts of a library work together correctly, units of code that work correctly on their own could have problems when integrated so test coverage of the integrated code is important as well, to create integration tests, need a tests directory

#### The tests Directory
- tests directory goes at the top level of proejct directory, next to src, Cargo knows to look for integration test files in this directory, can make as many test files and Cargo will lcompile each file as an individual crate
- Example: ```
      use lrs::add_two;

      #[test]
      fn it_adds_two() {
          let result = add_two(5);
          assert_eq!(result, 7);
      }```
- Each file in the tests directory is a separate crate, need to bring the library into each test crate's scope, need to bring code tested into scope
- Don't need to annotate any code in the file with `#[cfg(test)]`, Cargo treats the tests directory specially and compiles files in this directory only when running `cargo test`
- If any test in a section fails, the following sections will not be run, if a unit test fails, there won't be any output for integration and doc tests since those tests are only run if all unit tests are passing
- Each integration test file has its own section, adding more files into the tests directory will result in more integration test sections
- Can still run a particular integrtion test function by specifying the test function's name as an argument to `cargo test`, to run all the tests in a particular integration test file, use the `--test` argument of `cargo test` followed by the name of the file, for example `cargo test --test integration_test`

#### Submodules in Integration Tests
- With more integration tests, may want to make more files in the tests directory to help organize them, for example, can group test functions by the functionality they're testing, each file in the tests directory, each file in the tests directory is compiled as its own separate crate, which is useful for creating separate scopes to more closely imitate the way end users will be using crate, however, this means more files in teh tests directory don't share the same behavior as files in src do
- Different behavior of tests directory files is more noticeable given a set of helper functions to use in multiple integration test files and try to extract them into a common module, for example if creating a tests/common.rs and place a function named `setup` in it, can add some code to `setup` to call from multiple test files, having a `common` module appears in teh test results which is unintended, to avoid having `common` appear in the test output, insteaf of creating tests/common.rs, can create tests/common/mod.rs, this is the older convention that Rust also understands, naming the file this way indicates to Rust not ot treat the `common` module as an integration test file
- Naming the file this way indicates to not treat the `common` module as an integration test file
- After creating tests/common/mod.rs, can use it from any of the integrtion test files as a module

#### Integration Tests for Binary Crates
- If project is a binary crate that only contains a src/main.rs file and doesn't have a src/lib.rs file, can't create ingration tests in the tests directory and bring functions defined in the src/main.rs file into scope with a `use` statement, only library crates expose functions that other crates can use, binary crates are meant to be run on their own
- This is one of the reasons that Rust projects provide a binary and have a straightforward src/main.rs file that calls logic that lives in the src/lib.rs file, using that structure, integration tests can test the library crate with `use` to make the important functionality is available, if the important functionality works, the small amount of code in the src/main.rs will work as well and that small amount of code doesn't need to be tested

### Summary
- Rust's testing features provide a way to specify how code should function to ensure it continues to work as expected, even as changes are made, unit tests exercise different parts of a library separately and can test private implementation details, integration tests check that many parts of a library work together correctly and they use the library's public API to test the code in the same way external code will use it, despite Rust's type system and ownership rules to help prevent some kinds of bugs, tests are still important to reduce logic bugs having to do with how code is expected to behave
