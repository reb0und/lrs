# Fundamentals of Asynchronous Programming: Async. Await, Futures, and Streams
- Modern computers offer two tecnhiques for working on more than one operation at a time: parallelism and concurrency
- Writing programs that involve parallel or concurrent operations, challenges inherent to asynchronous operations are faced, where operations may not finish sequentially in the order they were started
- Alternative approach for asyncrhonous programming: Rust's Futures, Streams, the `async` and `await` syntax that supports them, and the tools for managing and coordinating between asynchronous operations
- Could avoid blocking main thread by spawning a dedicated thread to download each file, but overhead of threads would become a problem, would be preferable if the call didn't block in the first place, could also be better if could write same direct style used in blocking code, similar to:
   - ```let data = fetch_data_from(url).await;
      println!("{data}");```
- This is what Rust's async (asynchronous) abstraction provides

### Parallelism and Concurrency
- Concurrency is working on several different tasks before any of them is complete
- When a group of tasks is split up by having each member take one task and work on it alone is parallelism, each member makes progress at the exact same time
- In each workflow, may need to coordinate between different tasks
- Parallelism and concurrency can intersect each other, if something is blocked by a task, may want to focus efforts to unblock the other task, during this, both workers are unable to work in parallel, also no longer able to concurrently work on tasks
- Same dynamics come into play with software and hardware, on a machine with a single CPU core, the CPU can perform only one operation at a time, but can still work concurrently
   - Using tools such as threads, processes, and async, the computer can pause one activity and switch to others before eventually cycling back to that first activity againcycling 
   - On a machine with multiple CPU cores, can also do work in parallel, one core can be performing one task while another core performs a completely unrelated one and these operations happen at the same time
- When working with async in Rust, always dealing with concurrency, depending on hardware, operating system, and async runtime, may also use parallelism under the hood

## Futures and the Async Syntax
- The key elements of asynchronous programming in Rust are the `async` and `await` keywords
- A future is a value that may mot be ready now but will become ready at some point in the future
   - Rust provides a `Future` trait as a building block so that different async operations can be implemented with different data structures but with a common interface
   - Futures are types that implement the `Future` trait, each future holds its own information about the progress that has been made and what "ready" means
- Can apply the `async` keyword to blocks and functions to specify that they can be interrupted and resumed, within an async block or function, can use the `await` keyword to await a future (wait for it to become ready), any point where a future is awaited within an async block or function is a potential spot for that async block or function to pause and resume
   - The process of checking a future to see if its value is available yet is called polling
- When writing async Rust, the `async` and `await` keywords are used most of the time, Rust compiles them into equivalent code using the `Future` trait, much as it compiles `for` loops into equivalent code using the `Iterator` trait
- Since Rust provides the `Future` trait, can implement it for custom data types when needed
- Many functions return types with custom implementations of `Future` 

### First Async Program
- `trpl` is a crate that is short for "The Rust Programming Language", it re-exports all the types, traits, and functions needed (primarily from the `futures` and `tokio` crates
   - The `futures` crate is an official home for Rust experimentation for async code and where the `Future` trait was originally designed
   - Tokio is the most widely used async runtime in Rust today, especially for web applications
   - In some cases, `trpl` also renames or wraps the original APIs to remain focused on relevant materials
   - `cargo add trpl`
- Building a command tool to fetch two web pages, pull the `<title>` element from each, and print out the title of whichever page finishes that process first

### Defining the page_title function
- Writing a function that takes one page URL as a parameter, makes a request to it, and returns teh text of the title element
- Example: ```
async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}```
   - This defines a function `page_title` and marks it with the `async` keyword, then uses the `trpl::get` function to fetch whatever URL is passed in and adds the `await` keyword to await the response
   - To get the text of the response, the `text` method is called and is awaited with the `await` keyword
      - Both of these steps are asynchronous, for `get` function, need to wait for server to send back the first part of its response, including HTTP headers, cookies, etc and can be delivered separately of the response body
      - Especially if body is large, can take some time for text to arrive, must wait for entirety of response to arrive so `text` method is also async
- Must explicitly await both futures, since futures in Rust are lazy, they don't do anything until `await` keyword is used (Rust will show a compiler warning if a future is not used)
   - Similar to iterators in the sense that they do nothing until the `next` method is called (whether directly or by using `for` loops or methods such as `map` that use `next` under the hood)
   - Futures do nothing unless prompted, this laziness allows Rust to avoid running async code until its actually needed
- This is different from the behavior observed when using `thread::spawn`, where the closure passed to another thread starts running immediately, also different from how many other languages approach async, important for Rust to be able to provide its performance guarantees just as it is with iterators
- Once given the `response_text`, can parse it into an instance of the `Html` type using `Html::parse`, instead of a raw string, have a data type to use to work with the HTML as a richer data structure
- Can use the `select_first` method to find the first instance of a given CSS selector, by passing the string `"title"`, can get the first `<title>` element in the document if there is one, since there may not be any matching element, `select_first` returns an `Option<ElementRef>`, finally, can use the `Option::map` method, which allows work with the item in the `Option` if it's present, and do nothing if it isn't (would also use a `match` expression here but `map` is more idiomatic to get its content, which is a `String`), when all is done, result is an `Option<String>`
- Rust's `await` keyword goes after the expression being awaited, not before it, it's a postfix keyword, this makes chains of methods much nicer to work with, as a result, can change the body of `page_title` to chain the `trpl::get` and `text` function calls together with `await` between them
- Example: `let response_text = trpl::get(url).await.text().await;`
- When Rust sees a block with the `async` keyword, it compiles into a unique, anonymous data type that implements the `Future` trait, when Rust sees a function marked with `async`, it compiles into a non-async function whose body is an async block, an async function's return type is the type of the anonymous data type that the compiler creates for that async block
- Writing `async fn` is equivalent to writing a function that returns a future of the return type, to the compiler, a function definition such as the `async fn page_title` is equivalent to a non-async function defined like this: ```
fn page_title(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title_element| title_element.inner_html())
    }
}```
- This transformed function uses the `impl Trait` syntax, the returned type is a `Future` with an associated type of `Output`, the `Output` type is an `Option<String>`, this is the same as the original return type from the `async fn` version of `page_title`, all of the code in the body of the original function is wrapped in an `async move` block, blocks are expressions, the whole block is the expression returned from the function, this async block produces a value with the type `Option<String>`, that value matches the `Output` type in the return type, this is similar to other blocks previously seen, the new function body is an `async move` block because of how it uses the `url` parameter

### Determining a Single Page's Title
- 
