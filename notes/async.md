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
