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
- First, 
- Async allows concurrent code that is non blocking, to be written in a blocking manner
- Futures are values that are incomplete and will be ready in the future, `Future` is a trait that has many different implementations
- Code blocks or functions can be annotated with `async` to indicate they can be paused and resumed
- Within `async` blocks and functions, can use the `await` keyword to await a future (wait for the future to become ready)
- Any point where a future is awaited within an async block or function is a potential spot for the async block or function to pause and resume, the process of checking whether the value is ready is called polling
