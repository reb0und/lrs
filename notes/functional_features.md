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
- Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership, the closure will decide which of these to use based on what the body of the function does with the captured values
- Example: `let only_borrows = || println!("from closure: {list:?}");`
- This closure captures an immutable reference to the vector named `list` because it only needs an immutable reference to print the value
- This also indicates that a variable can bind to a closure definition and a closure can later be called by using the variable name and parentheses as if the variable name were a function name 
- Since there can be multiple immutable references to `list` at the same time, `list` is still accessible from the code before the closure definition, after the closure definition but before the closure is called, and after the closure is called
- Example: `let mut borrows_mutably = || list.push(1);`
- Closure body is changed so that it adds an element to the `list` vector, the closure now captures a mutable reference
- Can no longer have a `println!` between the definition and the call of the `borrows_mutably` closure, when `borrows_mutably` is defined, it captures a mutable reference to `list`, the closure is not used again after it is called so the mutable borrow ends
   - Between the closure definition and the closure call, an immutable borrow to print isn't allowed because no other borrows are allowed when there's a mutable borrow
- To force the closure to take ownership of the values it uses in the environment even through the body of the closure doesn't strictly need ownership, can use the `move` keyowrd before the parameter list
- Example: ```
    thread::spawn(move || println!("from thread {list:?}"))
        .join()
        .unwrap();```
- This is mostly useful when passing a closure to a new thread to move the data so that it's owned by the new thread
- Here a thread is spawned, giving the thread a closure to run as an argument, the closure body prints out the list
   - Previously, the closure only captured `list` using an immutable reference because that's the least amount of access to `list` needed to print it 
   - In this example, despite only needing an immutable reference, need to specify that `list` should be moved into the closure by putting the `move` keyword at the beginning of the closure definition, the new thread might finish before the rest of the main thread finishes of the main thread might finish first
   - If the main thread maintained ownership of `list` but ended before the new thread did and dropped `list`, the immutable reference in the thread would be invalid, therefore the compiler requires that `list` be moved into the closure given to the new thread so the reference will be valid


### Moving Captured Values out of Closures and the `Fn` Traits
- Once a closure has captured a reference or captured ownership of a value from the environment where the closure is defined (affecting what is moved into the closure), the code in the body of the closure defines what happens to the references or values when the closure is evaluated later (affecting what is moved out of the closure, a closure body can do any of the following:
   - Move a captured value out of the closure
   - Mutate the captured value
   - Neither move nor mutate the value
   - Capture nothing from the environment to begin with
- The way a closure captures and handles values from the environment affects which traits the closure implements, and the traits are how functions and structs can specify what kinds of closures can be used
- Closures will automatically implement one, two, or all three of these `Fn` traits, in an additive manner, depending on how the closure's body handles the values
1. `FnOnce` applies to closures that can be called once. All closures implement at least this trait because all closures can be called. A closure that moves captured values out of its body will only implement `FnOnce` and none of the other `Fn` traits because it can only be called once
2. `FnMut` applies to closures that don't move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
3. `Fn` applies to closures that don't move captured values out of their body and don't mutate captured values, as well as closures that capture nothing from their environment, these closures can be called more than once without mutating the environment, important in cases such as calling a closure multiple times concurrently
- Example: ```impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}```
- `T` is the generic type representing the type of the value in the `Some` variant of an `Optipn`, that type `T` is also the return type of the `unwrap_or_else` function, code that calls `unwrap_or_else` on an `Option<String>` will get a `String`
- The `unwrap_or_else` function has the additional generic type parameter `F`, the `F` type is the type of the parameter named `f` which is the closure provided when calling `unwrap_or_else`
- The trait bound specified on generic type `F` is `FnOnce() -> T`, meaning `F` must be able to be called once, take no arguments, and return a `T`, using `FnOnce` in the trait bound expresses the constraint that `unwrap_or_else` is going to call `f` at most one time, in the body of `unwrap_or_else`, can see that if the `Option` is `Some`, `f` won't be called, if the `Option` is `None`, `f` will be called once, since all closures implement `FnOnce`, `unwrap_or_else` accepts all three kinds of closures and is as flexible as it can be
- When not wanting to require capturing a value from the environment, can use the name of a function rather than a closure, example being `unwrap_or_else(Vec::new)` on an `Option<Vec<T>>` value to get a new empty vector if the value is `None`, the compiler automatically implements whichever of the `Fn` traits is applicable for a function definition
- Standard library method `sort_by_key` defined on slices differs from `unwrap_or_else` by using `FnMut` instead of `FnOnce` for the trait bound 
- Example `list.sort_by_key(|r| r.width);`
- The closure gets one argument in the form of a reference to the current item the slice being considered and returns a value of type `K` that can be ordered, this function is useful for sorting a slice by a particular attribute of each item
- The reason `sort_by_key` is defined to take an `FnMut` closure is that it calls the closure multiple times: once for each item in the slice, the closure `|r| r.width` doesn't capture, mutate, or move out anything from its environment, so it meets the trait bound requirements
- Example: 
```let mut sort_operations = vec![];
    let value = String::from("closure called");

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println!("{list:#?}");```
- This is an example of a closure that implements just the `FnOnce` trait since it moves a value out of the environmentm the compiler won't allow this closure with `sort_by_key`
- This is a poor attempt to try and count the number of times `sort_by_key` calls the closure when sorting `list`, this attempts to do so by pushing `value` (a `String` from the closure's environment) into the `sort_operations` vector, the closure captures `value` and then moves `value` out of the closure by transferring ownership of `value` to the `sort_operations` vector, this closure can be called once, trying to call it a second time wouldn't work because `value` would no longer be in the environment to be pushed into `sort_operations` again, therefore this closure only implements `FnOnce`, trying to compile this code would result in an error that `value` can't be moved out of the closure since the closure must implement `FnMut`
- Error points to line in closure body that moves `value` out of the environment, to fix, need to change the closure body so that it doesn't move values out of the environment and incrementing its value 
- Example:
```
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| { 
        num_sort_operations += 1;
        r.width 
    });```
- This is an alternative approach to count the number of times the closure is called, this closure works with `sort_by_key` because it is only capturing a mutable reference to the `num_sort_operations` counter and can therefore be called more than once
- `Fn` traits are important when defining or using functions or types that make use of closures

## Processing a Series of Items with Iterators
- The iterator pattern allows task to be performed on a sequence of items in turn, an iterator is responsible for the logic of iterating over each item and determining when the sequence has finished, when using iterators this logic does not need to be reimplemented
- In Rust, iterators are lazy, meaning they have no effect until methods are called that consume the iterator to use it up
- Example: ```
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();```
   - This creates an iterator over the items in the vector `v1` by calling the `iter` method defined on `Vec<T>`
   - The iterator is stored in the `v1_iter` variable, once created, it can be used in a variety of ways, iterating over an array with a `for` loop to execute some code on each of its items, under the hood this implicitly created and then consumed an iterator 
- Example: ```
    for val in v1_iter {
        println!("{val}");
    }```
   - Can separate the creation of the iterator from the use of the iterator in the `for` loop, when the `for` loop is called using the iterator in `v1_iter`, each element in the iterator is used in one iteration of the loop, printing out each value
- In other languages, would write this same functionality by starting a variable at index 0, using that variable to index into the vector to get a value, incrememting the variable value in a loop until it reached the total number of items in the vector
- Iterators handle all of this logic, cutting down on repetitive code, that could potentially be messed up, iterators also provide more flexibility to use the same logic with many different kinds of sequences, not just data structures that can be indexed into like vectors

### The `Iterator` Trait and the `next` Method
- All iterators implemenet a trait named `Iterator` that is defined in the standard library
- Example: ```
pub trait Iterator {
   type Item;

   fn next(&mut self) -> Option<Self::Item>;
}```
- This is an example of the definition of the `Iterator` trait
- This definition uses `type Item` and `Self::Item`, which define an associated type with this trait, implementing `Iterator` trait requires that an `Item` type is also defined and used in the return type of the `next` method, the `Item` type will also be the type returned from the iterator
- The `Iterator` trait only requires implementors to define one method: the `next` method, which returns one item of the iterator at a time, wrapped in `Some` and when the iteration is over, returns `None`
- Can call the `next` method on iterators directly
- Example: ```
assert_eq!(v1_iter.next(), Some(&1));
assert_eq!(v1_iter.next(), Some(&2));
assert_eq!(v1_iter.next(), None);```
- This demonstrates what values are returned from repeated calls to `next` on the iterator created from the vector
- Note that need to make `v1_iter` mutable: calling the `next` method on an iterator changes the internal state that the iterator uses to keep track of where it is in the sequence, in other words, this consumes or uses up the iterator. Each call to `next` eats up an item from the iterator, don't need to make `v1_iter` mutable  when using a `for` loop since the loop took ownership of `v1_iter` and made it mutable
- Also note that the values from calls to `next` are immutable references to the values in the vector, the `iter` method produces an iterator over immutable references, to create an iterator that takes ownership of `v1` and returns owned values, can call `into_iter` instead of `iter`, similarly, to iterate over mutable references, can call `iter_mut` instead of `iter`

### Methods That Consume the Iterator
- THe `iterator` trait has a number of different methods with default implementations provided by the standard library, some of these methods call the `next` method in their definition, which is why implementing the `next` method is required when implementing the `Iterator` trait
- Methods that call `next` are called consuming adapters because calling them uses up the iterator, one example is the `sum` method, which takes ownership of the iterator and iterates through the items by repeatedly calling `next`, consuming the iterator, as it iterates through, it adds each item to a running total and returns the total when the iteration is complete
- Example: ```
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);```
   - This is an example of the `sum` method
- Cannot use `v1_iter` after the call to `sum` because `sum` takes ownership of the iterator it's called on

### Methods that Produce Other Iterators
- Iterator adaptors are methods defined on the `Iterator` trait that don't consume the iterator, instead, producing different iterators by changing some aspect of the original iterator
- Example: `v1.iter().map(|x| x + 1);`
   - This calls the iterator adapter method `map`, which takes a closure call on each item as the items are iterated through, the `map` method returns a new iterator that produces the modified items, the closure then creates a new iterator in which each item from the vector will be incremented by 1
   - This specifies a closure that never gets called because iterator adapters are lazy and the iterator needs to be consumed here
   - To fix this and consume the iterator, can use the `collect` method that consumes the iterator and collects the resultant values into a collection data type
   - Example: `let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();`
      - The results of iterating over the iterator that's returned from the call to `map` into a vector are collected
- Since `map` takes a closure, can specify any operation to perform on each item, example of how closures allow customization of behavior while reusing the iteration behavior tbhat the `Iterator` trait provides
- Can chain multiple calls to iterator adapters to perform complex actions in a readable way, since iterators are lazy, have to call one of the consuming adapter methods to get results from calls to iterator adaptors

### Using Closures That Capture Their Environment
- Many iterator adapters take closures as arguments, and commonly the closures specified as arguments to iterator adapters will be closures that capture their environments
- `filter` iterator adapter method takes a closure that gets an item from the iterator and returns a `bool`, if the closure returns `true`, the value will be included in the iteration provided by `filter`, if the closure returns `false`, the value won't be included, if the closure returns `true`, the value will be included in the iteration provided by `filter`, if the closure returns `false`, the value won't be included
- Example: ```
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}```
    - `filter` is used with a closure that captures the `shoe_size`variable from its environment to iterate over a collection of `Shoe` struct instances, it will only return shoes that are the specified size
    - The `shoes_in_size` function takes ownership of a vector of shoes and a shoe size as parameters, it returns a vector containing only shoes of the specified size, in the body of `shoes_in_size`, `filter` is called to adapt that iterator into a new iterator that only contains elements for which the closure returns true, the closure captures the `shoe_size` parameter from the environment, keeping only shoes of the size specified, finally calling `collect` gathers the values returned by the adapted iterator into a vector that's returned by the function

## Improving the I/O Project
- Example: ```
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {```
- The standard library documentation for the `env::args` function shows that the type of the iterator it returns is `std::env::Args` and that type implements the `Iterator` trait and returns `String` values
- Have updated the signature of the `Config::build` function so the parameter `args` has a generic type with the trait bounds `impl Iterator<Item = String>` instead of `&[String]`, this usage of the `impl Trait` syntax means that `args` can be any type that implements the `Iterator` trait and returns `String` items
- Since ownership of `args` is being taken and `args` will be mutated by iterating over it, can add the `mut` keyword into the specification of the `args` parameter to make it mutable

### Using `Iterator` Trait Methods Instead of Indexing
- Example: ```
args.next();

let query = match args.next() {
    Some(arg) => arg,
    None => return Err("Missing query string");
}

let file_path = match args.next() {
    Some(arg) => arg,
    None => return Err("Missing file_path string");
}```
- Can call the `next` method since `args` implements the `Iterator` trait
- First value in the return value of `env::args` is the name of the program which is ignored and moves onto the next value, first call `next` and do nothing with the return value, then call `next` to get the value and put in the `query` field of `Config`, if `next` returns `Some`, a match is used to extract the value, if it returns `None`, not enough arguments were given and this returns early with an `Err` value, the same thing is done for `file_path` value

### Making Code Clearer with Iterator Adapters
- Can take advantage of iterators in the `search` function
- Can write code in a more concise way using iterator adapter methods, also avoids having a mutable intermediate `results` vector
- Functional programming style prefers to minimize amount of mutable state to make code clearer, removing the mutable state might enable a future enhancement to make searching happen in parallel, since no need to manage concurrent access to the `results` vector
- Example: ```
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents()
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}```
- Purpose of the `search` function is to return all lines in `contents` that contain the `query`, this code uses the `filter` adapter to keep only the lines for which `line.contains(query)` reutrns `true` for, then collect the matching lines into another vector with `collect`

### Choosing Between Loops or Iterators
- Instead of messing around with various bits of looping and building new vectors, code focuses on the high-level objective of the loop, abstracting away some of the commonplace code making it easier to see concepts unique to some code, such as the filtering condition each element in the iterator must pass
- Iterators get compiled down to roughly the same code as if writing the lower level code, iterators are a zero-cost abstraction, by which using the abstraction imposes no additional runtime overhead
