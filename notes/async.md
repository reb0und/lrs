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
- Writing a function that takes one page URL as a parameter, makes a request to it, and returns the text of the title element
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
    async {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title_element| title_element.inner_html())
    }
}```
- This transformed function uses the `impl Trait` syntax, the returned type is a `Future` with an associated type of `Output`, the `Output` type is an `Option<String>`, this is the same as the original return type from the `async fn` version of `page_title`, all of the code in the body of the original function is wrapped in an `async move` block, blocks are expressions, the whole block is the expression returned from the function, this async block produces a value with the type `Option<String>`, that value matches the `Output` type in the return type, this is similar to other blocks previously seen, the new function body is an `async move` block because of how it uses the `url` parameter

### Determining a Single Page's Title
- Example: ```
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url = &args[1];
    match page_title(url).await {
        Some(title) => println!("title for {url} is {title}"),
        None => println!("{url} had no title"),
    }
}```
- This gets the title for a single page, first passes the first URL `page_title` and await the result, since the value produced by the future is an `Option<String>`, need to use a `match` expression to print different messages to account for whether the page had a `<title>`
- Unfortunately, this code won't compile since `await` keywords can only be used in async functions and blocks but Rust won't allow the `main` function to be marked as async
- `main` cannot be marked as `async` because the async code needs a runtime: a Rust crate that manages the details of executing asyncrhonous code, a program's `main` function can initialize a runtime but it is not a runtime itself, every Rust program that executes async code has at least one place where it sets up a runtime and executes the futures
- Most languages that support async bundle a runtime, but Rust does not, instead there are many different async runtimes available, each of which make different tradeoffs suitable to the usecae it targets
   - A high-throughput web server has very different needs than a microcontroller with a single core, little RAM, and nok heap allocation ability
   - The crates that provide those runtimes also often supply async versions of common functionality such as file or network I/O
- Can use the `run` function from the `trpl` crate which takes a future as an argument and runs it to completion, behind the scenes, calling `run` sets up a runtime that's used to run the future passed in, once the future completes, `run` returns whatever the value the future produced
- Could pass the future returned by `page_title` directly to `run`, once complete, could match on the resulting `Option<String>`
- Example: ```
fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async move {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("title for {url} is {title}"),
            None => println!("{url} had no title"),
        }
    })
}```
- Each await point, every place where the code uses the `await` keyword, represents a place where control is handed back to the runtime, to make this work Rust needs to keep track of the state involved in the async block so that the runtime can execute some other work and then come back when it's ready to try advancing the first one again, this is a state machine
- An enum to save the current state at each await point: ```
enum PageTitleFuture<'a> {
   Initial { url: &'a str },
   GetAwaitPoint { url: &'a str },
   TextAwaitPoint { response: trpl::Response },
}```
- Writing code to transition between each state by hand would be tedious and error-prone, however, when needing to add more functionality and more states to the code later,
- The Rust compiler creates and manages the state machine data structures for async code automatically, normal borrowing and ownership rules around data structures all still apply, compiler also handles checking those
- Something has to execute the state machine, which is a runtime (an eexecutor is a part of a runtime responsible for executing the async code)
- If `main` were an async function, something else woul dneed to manage the state machine for whatever future `main` returned, but `main` is the starting point for the program, instead, this calls the `trpl::run` function in `main` to set up a runtime and run the future returned by the `async` block until it is done
- Some runtimes provide macros to write an async `main` function, those macros rewrite `async fn main { ... }` to be a normal `fn main` which calls a function that runts a future to completion the way `trpl::run` does

### Racing the Two URLs Together
- This calls `page_title` with two different URLs parsed in from the command line and races them
- Example: ```
fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let url_1 = &args[1];
        let url_2 = &args[2];

        let title_fut_1 = page_title(url_1);
        let title_fut_2 = page_title(url_2);

        let (url, maybe_title) =
            match trpl::race(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} finished first");

        match maybe_title {
            Some(title) => println!("title for {url} is {title}"),
            None => println!("{url} had no title"),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());

    (url, title)
}```
- This begins by calling `page_title` for each of the user supplied URLs, the resulting futures are saved as `title_fut_1` and `title_fut_2`, these futures are lazy and don't do anything until awaited, the futures are passed to `trpl::race` which returns a value to indicate which of the futures passed to it finishes first
- Under the hood, `race` is built on a more general function, `select`, which is encountered in real-world Rust code
   - A `select` function can do a lot of things that the `trpl::race` function can't, but has some added complexity that can be skipped over for now
- Either future can legitimately "win", so it does not make sense to return a `Result`, instead `race` returns `trpl::Either`, the `Either` type is somewhat similar to a `Result` in that it has two cases, but there is no notion of success or failure baked into `Either`, instead it uses `Left` and `Right` to indicate one or the other
- Example: ```
enum Either<A, B> {
   Left(A),
   Right(B),
}
- The `race` function returns `left` with output from the first future argument if it finishes first, or `Right` with the output of the second furture argument if that one finishes first, this matches the order the arguments appear in when calling the function, the first argument is to the left of the second argument
- `page_tile` is called with the same URL passed in, this way, the if the page returns first does not have a `<title>` to resolve, a meaningful message can still be printed, `println!` is updated to indicate which URL finished first and what, if any, the `<title>` is for the web page at that URL

## Applying Concurrency to Async
- Applying async to some concurrency challenges, differences between threads and futures
- In many cases, the APIs for working with concurrency using async are very similar to those for threads, in other cases they end up being quite different, even when the APIs look similar between threads and async, they often have different behaviora, nearly always have different performance characteristics

### Creating a New Task With `spawn_task`
- Counting to two on separate threads using async
- The `trpl` crate supplies a `spawn_task` function that is similar to the `thread::spawn` API and a `sleep` function that is an async version of the `thread::sleep` API
- Example: ```
use std::time::Duration;

fn main() {
    trpl::run(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("number {i} from the first task");
                trpl::sleep(Duration::from_millis(500)).await;       
            }
        });

        for i in 1..5 {
            println!("number {i} from the second task");
            trpl::sleep(Duration::from_millis(500)).await;       
        }
    });
}```
- Main function has `trpl::run` so that top level function can be async 
- Then, there are two loops within the block, each containing a `trpl::sleep` call which waits for 500 ms before sending the next message, one loop is placed in the body of a `trpl::spawn_task` and the other in a top-level `for` loop, an `await` is added after the `sleep` calls since they are futures
- Code behaves similarly to thread-based implementation, including the fact that may see messages appear in a different order in terminal when running 
- This version stops as soon as the `for` loop in the body of the main async block finishes, because the task spawned by `spawn_task` is shut down when the `main` function ends, to run all the way to task's completion, need to use a join handle to wait for the first task to complete
   - With threads can use the `join` method to block until the thread is done running, but with async, can use the `await` method to do the same thing, since the task handle itself is a future, its output type is a `Result`, need to unwrap after awaiting it
- Example: ```
fn main() {
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("{i} from spawned task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("{i} from main thread");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
    });
}```
- This will run until both loops finish
- So far, async and threads give same basic outcomes with different syntax, using `await` instead of calling `join` on the join handle, and awaiting the `sleep` calls
   - Bigger difference is that there was no need to spawn another operating system thread to do this, in fact don't need to spawn a task here, since async blocks compile to anonymous futures, can put each loop in an async block and have the runtime run them both to completion using the `trpl::join` function
   - Previously, used the `join` method on the `JoinHandle` type returned by calling `std::thread::spawn`, the `trpl::join` function is similar, but for futures
      - When given two futures, it produces a single new future whose output is a tuple containing the output of each future passed in once they both complete
      - `trpl::join` can be used to wait for both `fut1` and `fut2` to finish, `fut1` and `fut2` are not awaited but instead, the new future produced by `trpl::join` is awaited
      - The output is ignored since it's just a tuple containing two unit values
      - Example: ```
fn main() {
    trpl::run(async {
        let fut1 = async {
            for i in 1..10 {
                println!("{i} from spawned task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("{i} from main thread");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });
}```
- Now will see the same order every time, which is different from with threads, since the `trpl::join` function is fair, meaning it checks each future equally often, alternating between them, and never lets one race ahead if the other is ready, with threads, the operating system bdecies which thread to check and how long to let it run, with async Rust, the runtime decies which task to check
   - In practice, the details get complicated because an async runtime might use operating system threads under the hood as part of how  it manages concurrency, guaranteeing fairness can be more work for a runtime but it's still possible
   - Runtimes don't have to guarantee fairness for any given operation and they often offer differnet APIs to choose whether or not to provide fairness

#### Counting Up on Two Tasks Using Message Passing
- Sharing data between futures will also be familiar, will use message passing again, but this time with async versions of the types and functions
- Example: ```
fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("got {received}");
    });
}```
- Single async block used, not spawning a separate task like a separate thread
- Here, used `trpl::channel`, an async version of the multiple-producer, single-consumer channel API used with threads previously
- The async version of the API is only a little different from the thread-based version, it uses a mutable rather than an immutable receiver, `rx` and its `recv` method produces a future to await rather than producing the value directly, now can send messages from the sender to the receiver, don't have to spawn a separate thread of reven a task, only need to await the `rx.recv` call
- The syncrhonous `Receiver::recv` method in `std::mspc::channel` blocks until it receives a message, the `trpl::Recevier::recv` method does not, since it is async, instead of blocking, it hands control back to the runtime until either a message is received or the send side of the channel closes
- Don't need to await the `send` call because it doesn't block, it doesn't need to since the channel it sends to is unbounded
- Since all async code runs in an async block in a `trpl::run` call, everything with in it can avoid blocking, however the code outside it will block on the `run` function returning
   - The `trpl::run` function allows the choice of where to block on some set of async code and thus where to transition between sync and async code, in most async runtimes, `run` is actually named `block_on` for exactly this reason
- In this example, the message will arrive right away, although a future could be sued, here, there is no concurrency yet, everything happens in sequence, just as if there were no futures involved
- Example: ```
use std::time::Duration;

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("a"),
            String::from("abc"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(value) = rx.recv().await {
            println!("got {value}");
        }
    });
}```
- This sends a series of messages and sleeps in between them
- Along with sending the messags, need to receive them, in this case, since it is known how many messages are coming in, could to this manually by calling `rx.recv().await` four times, but would be unknown num ber in the real world, need to keep waiting until it is determined that there are no more messages
- A `for` loop is used to process all the items received from a syncrhonous channel, Rust doesn't have a way to loop over an asyncrhonous series of items, need to use a `while let` conditional loop, this is the loop version of the `if let` construct, the loop will continue executing as long as the pattern it specifies continues to match the value
- The `rx.recv` call produces a future, which is awaited, the runtime will pause the future until it is ready, once a message arrives, the future will resolve to `Some(message)`, as many times as a message arrives, when the channel closes, regardless of whether any messages have arrived, the future will instead resolve to `None` to indicate there are no more values and thus should stop polling or awaiting
- The `while let` loop pulls all of this together, if the result of calling `rx.recv().await` is `Some(message)`, gain access to the message and can use it in the loop body, just as done with `if let`, if the result is `None`, the loop ends, each time the loop completes, it hits the await point again, so the runtime pauses it again until another message arrives
- The code now successfully sends and receives all of the messages, but there are still a couple of problems, messages do not arrive at half-second intervals, they arrive all at once, a few seconds after starting the program, other problem is the program never exits
- To get desired behavior, need to put the `tx` and `rx` operations in their own async blocks, then the runtime can execute each of them separetly using `trpl::join`, once again, await the result of calling `trpl::join`, not the individual futures, if awaiting the individual futures in sequence, would just end up back in a sequential flow
- Example: ```
use std::time::Duration;

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async {

            let vals = vec![
                String::from("hi"),
                String::from("a"),
                String::from("abc"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let recv_fut = async {
            while let Some(value) = rx.recv().await {
                println!("got {value}");
            }
        };

        trpl::join(tx_fut, recv_fut).await;
    });
}```
- With updated code, messages get printed at 500 ms intervals rather than all in a rush after 2s
- The program still never exits though, since the way the `while let` loop interats with `trpl::join`
   - The future returned from `trpl::join` completes only once both futures passed to it have completed
   - The `tx` future completes once it finishes sleeping after sending the last message in `vals`
   - The `rx` future won't complete until the `while let` loop ends
   - The `while let` loop won't end until awaiting `rx.recv` produces `None`
   - Awaiting `rx.recv` will return `None` only once the other end of the channel is closed
   - The channel will close only if called `rx.close` or when the sender side `tx` is dropped
   - `rx.close` is not called anywhere and `tx` won't be dropped until the otuermost aysnc block passed to `trpl::run` ends
   - The block can't end because it is blocked on `trpl::join` completing, which takes back to the top of this list
   - Could manually close `rx` by calling `rx.close` somewhere, but that doesn't make much sense, stopping after handling some arbitrary number of messages would make the program shut down, could miss messages, need some other way to make sure that `tx` gets dropped before the end of the function
- Right now, the async block that sends the messages only borrows `tx` because sending a message doesn't require ownership, but if `tx` could be moved into the async block, it would be dropped once that block ends, can use the `move` keyword with async blocks
- Can change the block used to send messages from `async` to `async move`, this version of code will gracefully shut down after last message is sent and received
- Example: ```
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("a"),
                String::from("abc"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };```
- Since async channel is a multiple-producer channel, can call `clone` on `tx` to send messages from multiple futures
- Example: ```
use std::time::Duration;

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("a"),
                String::from("abc"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let recv_fut = async {
            while let Some(value) = rx.recv().await {
                println!("got {value}");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("a"),
                String::from("b"),
                String::from("c"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        trpl::join3(tx1_fut, recv_fut, tx_fut).await;
    });
}```
- First, `tx` is cloned, creating `tx1` outside the first async block,, `tx1` is moved into that block just as done before with `tx`, later, the original `tx` is moved into another async block, where more messages are sent on a slower delay, then the new async block is placed after the async block for receiver messages, this could go before it just as well, the key is the order in which the futures are awaited, not in which they're created
- Both of the async blocks for sending messages need to be `async move` blocks so that both `tx` and `tx1` get dropped when those blocks finish, otherwise, will end up back in an infinite loop, then switch from `trpl::join` to `trpl::join3` to handle the additional future
- Now all the messages are seen from both sending futures, and since the sending futures use slightly different delays, the messages are received at different intervals

## Working with Any Number of Futures
- There is a macro form of `join` to which an arbitrary number of arguments can be passed, it handles awaiting for the futures itself
   - Example: `trpl::join!(tx1_fut, recv_fut, tx_fut);`
- Mcaro form only works when the number of futures is known ahead of time
- Pushing futures to a collection and then waiting on some or all of the futures to complete is a common pattern
- To check all of the futures in some collection, need to iterate and join on all of them, the `trpl::join_all` function accepts any type that implements the `Iterator` trait
- Example: ```
let futures = vec![tx1_fut, recv_fut, tx_fut];
trpl::join_all(futures);```
- This code does not compile 
- None of the async blocks return anything, each one produces a `Future<Output = ()>`, since `Future` is a trait though and the compiler creates a unqiue enum for each async block, can't put two different hand-written structs in a `Vec` and the same rule applies to the different enums generated by the compiler
- To make this work, need to use trait objects that allow treating each of the anonymous functions produced by these types as the same type since all of them implement the future trait
- There is another way to include multiple types in a `Vec`: using an enum to represent each type that can appear in the vector, can't do that here, no way to name the different types since they are anonymous, also the only reason a vector was used with `join_all` was to be able to work with a dynamic collection of futures where only care about the same output type
- Can try to wrap each future in the `vec!` in a `Box::new`
- Example: `let futures = vec![Box::new(tx1_fut), Box::new(recv_fut), Box::new(tx_fut)];`
- This code still doesn't compile, will get the same error and new erorrs referring to the `Unpin` trait
- Can fix the type errors on the `Box::new` calls by explicitly annotating the type of the `futures` variable 
- Example: `let futures: Vec<Box<dyn Future<Output = ()>>> = vec![Box::new(tx1_fut), Box::new(recv_fut), Box::new(tx_fut)];`
   - The innermost type is the future itself, need to explicitly note that the output of the future is the unit type `()` by writing `Future<Output = ()>`
   - Then need to annotate the trait with `dyn` to mark it as dynamic
   - The entire trait reference is wrapped in a `Box`
   - Need to explicitly state that `futures` is a `Vec` containing these items
- Now only get errors mentioning `Unpin`, async block does not implement the `Unpin` trait and suggests using `pin!` or `Box::pin` to resolve it, can follow compilers advice for now, importing `Pin` from `std::pin` and updating the type annotation for `futures` with a `Pin` wrapping each `Box`, can use `Box::pin` to pin the futures themselves
- Example: `let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![Box::pin(tx1_fut), Box::pin(recv_fut), Box::pin(tx_fut)];`
- Using `Pin<Box<T>>` adds a small amount of overhead from putting these futures on the heap with `Box`, only doing that to get the types to line up, dont actually need the heap allocation, these futures are local to this particular function, `Pin` is a wrapper type, can get the benefit of having a single type in the `Vec`, the original reason for `Box`, without doing a heap allocation, can use `Pin` directly with each future, using the `std::pin::pin` macro
- Must be explicit about the type of the pinned reference, otherwise, Rust will still not know to interpret these as dynamic trait objects, which is what they need to be in the `Vec`, therefore need to add `pin` to the list of imports from `std::pin`, can `pin!` each future when defining it and define `futures` as a `Vec` containing pinned mutable references to the dynamic future type
- Example: ```
use std::time::Duration;
use std::pin::{Pin, pin};

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("a"),
                String::from("abc"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("got {value}");
            }
        });

        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("a"),
                String::from("b"),
                String::from("c"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
            vec![tx1_fut, rx_fut, tx_fut];

        trpl::join_all(futures).await;
    });
}```
- Need to mutably borrow future to take the future forward in state
- May have different `Output` types, and in this case will need to use the `join!` macro since `join_all` requires that the same output type exists, can either work with a dynamic number of futures with `join_all` as long as they have the same type, or can deal with a set of number of futures with the `join` functions or the `join!` macro if they have different types

### Racing Futures
- When futures are joined with the `join` family of functions and macros, all of them must be finished before moving on, sometimes, only need some future from a set to finish before moving on, similar to racing one future against another
- Example: ```
use std::time::Duration;

fn main() {
    trpl::run(async {
        let slow = async {
            println!("slow started");
            trpl::sleep(Duration::from_millis(500)).await;
            println!("slow finished");
        };

        let fast = async {
            println!("fast started");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("fast ended"):
        };

        trpl::race(slow, fast).await;
    });
}```
- Each future prints a message when it starts running, pauses for some amount of time by calling and awaiting `sleep`, then prints another message when it finishes, then both `slow` and `fast` are passed to `trpl::race` and wait for one of them to finish, can ignore the `Either` instace returned, since all of the interesting behavior happens in the body of the async blocks
- If the order of the arguments to `race` are flipped, the order of the started messages changes, even though the `fast` future always completes first, this is because of the implementation of this particular `race` function is not fair, it always runs the futures in the arguments in the order in which they're passed, other implementations are fair and wil randomly choose which future to poll first, regardless of whether the implementation of race used is fair, one of the futures will run up to the first `await` in its body before another task can start
- At each await point, Rust gives a runtime a chance to pause the task and swtich to another one if the future being awaited isn't ready, the inverse is also true, Rust only pauses async blocks and hands control back to a runtime at an await point, eveything between await points is synchronous
- This means if a bunch of work is done in an async block without an await point, that future will block any other future from making process, this can be thought of as one future starving other futures, in some cases, this may not be a big deal, but if doing some kind of expensive setup or long-running work, or if there is a future that will keep doing some particular task indefinitely, need to think about when and where to hand control back to the runtime
- If thwre are long-running blocking operations, async can be a useful tool for providing ways for different parts of the program to relate to each other

### Yielding Control to the Runtime
- Here is a slow function, uses `std::thread::sleep` instead of `trpl::sleep` so that calling `slow` will block the current thread for some number of ms, can use `slow` to stand in for real-world operations that are both long-running and blocking
- Example: ```
use std::time::Duration;
use std::thread;

fn main() {
    trpl::run(async {
        let a = async {
            println!("a started");
            slow("a", 20);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("a finished");
        };

        let b = async {
            println!("b started");
            slow("b", 200);
            slow("b", 100);
            slow("b", 200);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("b finished");
        };

        trpl::race(a, b).await;
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name} ran for {ms}");
}```
- Each future only hands control back to tbe runtime after carrying out a bunch of slow operations
- `race` still finishes as soon as `a` is done, there is no interleaving between the two futures
- The `a` future does all of its work, until the `trpl::sleep` call is awaited, then the future `b` does all of its work until its own `trpl::sleep` call is awaited, and finally the `a` future completes, to allow both futures to make progress between their slow tasks, need await points to hand control back to the runtime, this means something can that can be awaited
- This kind of handoff would happen if `trpl::sleep` was removed at the end of the `a` future, it would complete without the `b` future running at all, can use the `sleep` function as a starting point for letting operations switch off making progress
- Example: `trpl::sleep(Duration::from_millis(1)).await;`
- Adding the `trpl::sleep` calls with awit points between each call to`slow` makes the two futures' work interleaved
- The `a` future still runs for a bit before handing off control to `b` because it calls `slow` before ever calling `trpl::sleep`, but after that the futures swap back and forth each time one of them hits an await point, could have done this after every call to `slow` but could break up the work in whatever way makes the most sense
- Example: `trpl::yield_now().await;`
- This is clearer about the actual intent and can be significantly faster than using `sleep` since timers such as the one used by `sleep` often have limits on how granular they can be, the version of `sleep` in use will always sleep for at least a millisecond, even if its passed a `Duration` of one nanosecond
- Example: ```
use std::time::{Instant, Duration};

fn main() {
    trpl::run(async {
        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::sleep(Duration::from_nanos(1)).await;
            }
        }.await;

        let time = Instant::now() - start;
        println!(
            "sleep version finished after {} secs", 
            time.as_secs_f32()
        );

        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::yield_now().await;
            }
        }.await;
        let time = Instant::now() - start;
        println!("yield version finished in {}", time.as_secs_f32());
    });
}```
- Here, all of the status printing is skipped, passing a one-nanosecond `Duration` to `trpl::sleep` and let each future run by itself with no switching between the futures, they run for 1000 iterations and see how long the future using `trpl::sleep` takes compared to the future using `trpl::yield_now`
- This means that async can be useful for even compute-bound tasks, depending on what the program is doing, since it provides a useful tool for structuring the relationships between different parts of the program, this is a form of cooperative multitasking, each future has the power to determine when it hands over control via await points, each future therefore also has the responsibility to avoid blocking for too long, in some Rust-based embedded operating systems, this is the only kind of multitasking
- In real-world code, won't usually be alternating function calls with await points on every single line, while yielding control this way is relatively inexpensive, it's not fre, in many cases, trying to break up a compute-bound task might make it significantly slower, sometimes is better for overall performance to let an operation block briefly, always  measure to see what code's actual performance bottlenecks are
- The underlying dynamic is important to keep in mind, if seeing a lot of work happening in serial that should be happening concurrently

### Building Custom Async Abstraction
- Can also compose futures to create new patterns, for example, can build a `timeout` function with async building blocks alrady known, when done, the result will be another building block to use to create more async abstractions
- Example: ```
use std::time::Duration;

fn main() {
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_millis(100)).await;
            "done"
        };

        match timeout(slow, Duration::from_millis(10)).await {
            Ok(message) => println!("succeeded with {message}"),
            Err(duration) => {
                println!("failed after {} seconds", duration.as_secs())
            }
        }
    });
}```
- To implement this, need to consider the API for `timeout`:
- Needs to be an async function to await it
- First parameter should be a future to run, can make it generic to allow it to work with any future
- Its second parameter should be the maximum time to wait, can use a `Duration` that will make it easy to pass along to `trpl::sleep`
- Should return a `Result`, if the future completes successfully, the `Result` will be `Ok` with the value produced by the future, if the timeout elapses first, the `Result` will be `Err` with the duration that the timeout waited for
- Example: ```
async fn timeout<F: Future>(
    future_to_try: F, 
    max_time: Duration
) -> Result<F::Output, Duration> {}```
- Need to race the future passed in against the duration, can use `trpl::sleep` to make a timer future from the duration and use trpl::race to run the timer with the future the caller passes in
- Also know that `race` is not fair, polling arguments in the order in which they are passed, thus, `future_to_try` is passed to `race` first so it gets a chance to complete even if `max_time` is a very short duration, if `future_to_try` finishes first, `race` will return `Left` with the output from `future_to_try`, if `timer` finishes first, `race` will return `Right` with the timer's output of `()`
- Example: ```
use std::time::Duration;
use trpl::Either;

fn main() {
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_millis(100)).await;
            "done"
        };

        match timeout(slow, Duration::from_millis(10)).await {
            Ok(message) => println!("succeeded with {message}"),
            Err(duration) => {
                println!("failed after {} seconds", duration.as_secs())
            }
        }
    });
}

async fn timeout<F: Future>(
    future_to_try: F, 
    max_time: Duration
) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}```
- If the `future_to_try` succeeds and results in a `Left(output)`, return `Ok(Output)`, if the sleep timer elapses instead, receive a `Right(())`, igore the `()` with `_` and return `Err(max_time)` instead
- With this, there is a working `timeout` built out of two other async helpers
- Since futures compose with other futures, can build really powerful tools using smaller async building blocks, can use this same approach to combine timetouts with retires, and in turn use those with operations such as network calls
- In practice, will usually work directly with `async` and `await`, and secondarily with functions and macros such as `join`, `join_all`, `race`, and so on, will only need to reach for `pin` now and again to use futures with those APIs
- Have now seen a number of ways to work with multiple futures at the same time, next will look at how to work with multiple futures in a sequence over time with streams, also should consider:
   - How could a `Vec` be used to process a group of futures in sequence instead, what are the tradeoffs of doing that?
   - `futures::stream::FuturesUnordered` type from `futures` crate, how would this be different if using a `Vec`

## Streams: Futures in Sequence
- The async `recv` method produces a sequence of items over time, this is an instance of a much more general pttern known as a stream
- There are two differences between iterators and the async channel receiver, the first difference is time: iterators are synchronous, while the channel receiver is asynchronous, second is the API, when working directly with `Iterator`, can call its syncrhonous `next` method, with the `trpl::Receiver` stream in particular, called an asynchronous `recv` method instead
- Otherwise, these APIs feel very similar and that similarity isn't a coincidence, a stream is like an asyncrhonous form of iteration, whereas the `trpl::Receiver` specifically waits to receive messages, though, the general-purpose stream API is much broader, it provides the next item in the way `Iterator` does, but asynchronously
- The similarlity between iterators and streams in Rust means can actually create a stream from any iterator, as with an iterator can work with a stream by callking its `next` method and then awaiting the output
- Example: ```
fn main() {
    trpl::run(async {
        let values = [1, 2, 3, 4, 5];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("the value was: {value}");
        }
    });
}```
- This is an array of numbers, converted to an iterator, the called `map` on to double all values, then iterator is converted into a stream using the `trpl::stream_from_iter` function, next this loops over the items in the stream as they arrive with the `while let` loop
- This code does not compile, notes that there is no `next` method available, the reason for this compiler error is that the right trait is needed in scope to be able to use the `next` method, right trait may be expected to be `Stream` but is actually `StreamExt`, short for extension, `Ext` is a common pattern in the Rust community for extending one trait with another
- `Stream` trait defines a low-level interface that effectively combines the `Iterator` and `Future` traits
- `StreamExt` supplies a higher-level set of APIs on top of `Stream`, including the `next` method as well as other utility methods similar to those provided by the `Iterator` trait, `Stream` and `StreamExt` are not yet part of Rust's standard library but most ecosystem crates use the same definition
- Can fix the compiler error by adding a `use` statement for `trpl::StreamExt`
- Example: `use trpl::StreamExt;`
- With all of those pieces together, this code works as expected, with with `StreamExt` in scope, can use all of its utility methods, just as with iterators, for example, can use the `filter` method to filter out everything but multiples of three and five
- Example: ```
use trpl::StreamExt;

fn main() {
    trpl::run(async {
        let values = [1, 2, 3, 4, 5];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        let mut filtered =
            stream.filter(|value| value % 3 == 0 || value % 5 == 0);

        while let Some(value) = filtered.next().await {
            println!("the value was: {value}");
        }
    });
}```
- Could do the same with normal iterators and without any async at all

### Composing Streams
- Many concepts are naturally represented as streams: items becoming available in a queue, chunks of data being pulled incrementally from the filesystem when the full data set is too large for the computer's memory, or data arriving over the network over time
- Since streams are futures, can use them with any kind of future and combine them in interesitng ways, for example, can batch up events to avoid triggering too many network calls, set timeouts on sequences of long-running operations, or throttle user interface events to avoid doing needless work
- Can start by building a little stream out of messages as a stand-in for a stream of data seen from a websocket
- Example: ```
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let mut messages = get_messages();

        while let Some(message) = messages.next().await {
            println!("{message}");
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messages = ["a", "b", "c", "d", "e"];
    for message in messages {
        tx.send(format!("message: {message}")).unwrap();
    }
    
    ReceiverStream::new(rx)
}```
- Fist, a function called `get_messages` is created that returns `impl Stream<Item = String>`, for its implementation, can create an async channel, loop over the first 5 letters of the English alphabet, and send them across the channel
- Can also use a new type `ReceiverStream` which converts the `rx` recevier from the `trpl::channel` into a `Stream` with a `next` method, in `main`, used a `while let` loop to print all the messages from the stream
- Could do this with the regular `Receiver` API or even the regular `Iterator` API, though, a feature that requires streams could be adding a timeout that applies to every item in the stream
- Example: ```
use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => println!("error: {reason}"),
            }
        }
    })
}```
- Start by adding a timeout to the stream with the `timeout` method which comes from the `SreamExt` trait, then the body of the `while let` loop is updated, because the stream now returns a result, the `Ok` variant indicates a message arrived in time, the `Err` variant indicates that the timeout elapsed beforea any message arrived, the result is placed in a `match` statement and either print the message when received successfully or print a notice about the timeout, finally, the messages are pinned after applying the timeout to them, because the timeout helper produces a stream that needs to be pinned to be polled
- Since there are no delays between messages, the timeout does not change the behavior of the program
- Example: ```
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            tx.send(format!("message: {message}")).unwrap();
        }
    });

    ReceiverStream::new(rx)
}```
- In get messages, used `enumerate` Iterator method with the `messages` array so that can get the index of each item sending along with the item itself, then applied a 100 ms delay to even-index items and a 300 ms delay to the odd index-items to simulate the different delays seen from a stream of messages in the real world, since the timeout is for 200 ms, this should affect half the messages
- To sleep between messages in the `get_messages` function without blocking, need to use async, can't make `get_messages` itself into an async function, because then it would return a `Future<Output = Stream<Item = String>>`, instead of a `Stream<Item = String>>`, the caller would have to await `get_messages` itself to get access to the stream, note that everything in a given future happens linearly, concurrency happens between futures, awaiting `get_messages` would require it to send all the messages, including the sleep delay between each message before returning the receiver stream and as a result the timeout would be useless, there would be no delays in teh stream itself
- Instead, `get_messages` is left as a regular function that returns a stream, and spawns a task to handle the async `sleep` calls
- Calling `spawn_task` in this way works because the runtime has already been set up, otherwise, would result in a panic, other implementations choose different tradeoffs, they might spawn a new runtime and avoid the panic but end up with a bit of extra overhead, or they may not provide a standalone way to spawn tasks without reference to a runtime
- The timeout doesn't prevent the messages from arriving in the end, still get all of the original messages, since hte channel is unbounded: it can hold as many messages as can fit into memory, if the message doesn't arrive before the timout, the stream handler will account for that but when it pools the stream again, it may now have arrived
- Can get different behavior using other kinds of channels or other kinds of streams more generally, can combine a stream of time interals with this stream of messages

### Merging Streams
- Can create another stream, which will emit an item every ms if it is allowed to run directly, for simplicity, can use the `sleep` function to send a message on a delay and combine it with the same approach used in `get_messages` of creating a stream from a channel, the difference is that this time, need to send back the count of intervals that have elapsed, to the return type will be `impl Stream<Item = u32>`, can call the `get_intervals` function
- Example: ```
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            tx.send(count).unwrap();
        }
    });

    ReceiverStream::new(rx)
}```
- Will start by defining a `count` in the task, can define it outside the task, too, but it's clearer to limit the scope of any given variable, then creating an infinite loop, each iteration of the loop sychronously sleeps for one ms, increments the count, and then sends it over the channel, this is all wrapped in the task created by `spawn_task`, all, including the infinite loop, will get cleaned up along with the runtime
- This kind of infinite loop, that ends only when the whole runtime gets torn down, is fairly common in async Rust, many programs need to keep running indefinitely, with async, this doesn't block anything else, as long as there is at least one await point in each iteration through the loop
- Now, in main function's async block, can attempt to merge the `messages` and `intervals` streams
- Example: ```
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals();
        let merged = messages.merge(intervals);```
- Can start by calling `get_intervals`, then merge the `messages` and `intervals` streams with the `merge` method, which combines multiple streams into one stream that produces items from any of the source streams as soon as items are available, without imposing any particular ordering, the combined stream is looped over instead  of over `messages`
- At this point, neither `messages` nor `intervals` needs to be pinned or mutable, despite this, the call to `merge` doesn't compile, neither does the `next` call in the `while let` loop, this is because the two streams have different types, the `messages` stream has the type `Timeout<impl Stream<Item = String>>`, where `Timeout` is the type that implements `Stream` for a timeout call, the `intervals` stream has the typle `impl Stream<Item = u32>`, to merge these two streams, need to transform one of them to match the other, can rework teh intervals stream because messages is already the basic format wanted and has to handle timeout errors
- Example: ```
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("interval: {count}"))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals);

        let mut stream = pin!(merged);```
- Can use the `map` helper method to transform the `intervals` into a string, second, need to match the `Timeout` from `messages`, don't actually want a timeout for `intervals`, can just create a timeout which is longer than the other durations used, here, can create a 10s timeout with `Duration::from_secs(10)`, finally, need to make `stream` mutable so that the `while let` loop's `next` calls can iterate through the stream and pin it so that it's safe to do so, that gets almost to desired position, first problem is that it never stops, second is that messages are buried in midst of interval counter
- Example: ```
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);```
- First, can use the `throttle` method on the `intervals` stream so that it doesn't overwhelm the `messages` stream, throttling is a way of limiting the rate at which a function will be called, or, how often the stream will be polled, once every 100 ms should do (how often messages arrive)
- To limit the number of items, will accept from stream the `take` method is aplied to the `merged` stream, to limit the final output, not just the one stream or the other
- Now, when running the program, it stops after pulling 20 items from the stream, and the intervals don't overwhelm the messages, also don't get `Interval: 100` or `Interval: 200` or so on, instead get `Interval: 1`, `Interval: 2` and so on, despite having a source stream tha tcan produce an event every ms, this is because the `throttle` call produces a new stream that wraps the original stream so that the original stream gets polled only at the throttle rate, not its own "native" rate, don't have a bunch of unhandled interval messages that are chosen to be ignored, instead, they are never produced in the first place, this is the inherent "laziness" of Rust's futures at work again, allowing the selection of performance characteristics
- Errors need to be handled, with both of these channel-based streams, the `send` calls could fail when the other side of the channel closes, that's just a matter of how the runtime executes the futures that make up the stream, up until now, this possibility has been ignored by calling `unwrap`, but in a well-behaved app, should explicitly handle the error, at minimum by ending the loop so no more messages are tried to be sent, can print the issue and then `break` from the loops
- ```
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("message: {message}")) {
                eprintln!("could not send interval {message}: {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(100)).await;
            count += 1;
            if let Err(send_error) = tx.send(count) {
                eprintln!("could not send interval {count}: {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}```
- Correct way to handle a message send error will vary

## A Closer Look at the Traits for Async
-  `Future`, `Pin`, `Unpin`, `Stream`, and `StreamExt` traits have been used in various ways, need to understand more details regarding these

### The `Future` Trait
- Rust's definition of the `Future` trait:
- Example: ```
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}```
- `Future`'s associated type `Output` says what the future resolves to, this is analogous to the `Item` associated type for the `Iterator` trait
- `Future` also has the `poll` method which takes a special `Pin` reference for its `self` parameter and a mutable reference to a `Context` type and returns a `Poll<Self::Output>`
- Example: ```
enum Poll<T> {
    Ready(T),
    Pending,
}```
- This `Poll` type is similar to an `Option`, it has one variant and has a value, `Ready(T)` and one which does not, `Pending`, `Poll` means something quite different from `Option`, though the `Pending` variant indicates that the future has work to do, so the caller will need to check again later, the `Ready` variant indicates that the future has finished its work and the `T` value is available
- With most futures, the caller should not call `poll` again after the future has returned `Ready`, many futures will panic if polled after becoming ready, Futures that are safe to poll again will explicitly say so in documentation, similar to how `Iterator::next` behaves
- When there is code that uses `await`, Rust compiles it under the hood to code that calls `poll`
- Rust compiles code that prints out page title for a single URL when it resolves into this:
```
match page_title(url).poll() {
    Ready(page_title) => match page_title {
        Some(title) => println!("title for {url} was {title}"),
        None => println!("{url} had no title"),
    },
    Pending => {
        // ?
    }
}```
- What should be done when the future is still `Pending`, need some way to repeatedly try (loop)
- Example: ```
    loop {
        match page_title(url).poll() {
            Ready(page_title) => match page_title {
                Some(title) => println!("title for {url} was {title}"),
                None => println!("{url} had no title"),
            },
            Pending => {
                // ?
            }
        }
    }```
- If Rust compiled to exactly this code, every `await` would be blocking, the opposite of the intention here, Rust makes sure that the loop can hand off control to something that can pause work on this future to work on other futures then check this one again later, that something is an async runtime, and this scheduling and coordination work is one of its main jobs
- Earlier, described waiting on `rx.recv`, the `recv` call returns a future, and awaiting the future polls it, runtime will pause the future until it's ready with either `Some(message)` or `None` when the channel closes, can see how this works with `Future::poll`, the runtime knows the future isn't ready when it returns `Poll::Pending`, conversely, the runtime knows the future is ready and advances when `poll` returns `Poll::Ready(Some(message))` or `Poll(Ready(None))`
- A runtime polls each future it is responsible for, putting the future back to sleep when it is not yet ready

### The `Pin` and `Unpin` Traits
- The `trpl::join_all` function returns a struct called `JoinAll`, that struct is a generic over a type `F`, whcih is constrained to implement the `Future` trait, directly awaiting a future with `await` pins the future implicitly, which is why `pin!` is not needed everywhere a future needs to be awaited
- Not directly awaiting a future here, instead construct a new future, `JoinAll`, by passing a collection of futures to the `join_all` function, the signature for `join_all` requires that the types of the items in the collection all implement the `Future` trait, and `Box<T>` implements `Future` only if the `T` it wrapps is a future that implements the `Unpin` trait
- The `cx` parameter in the `poll` method and its `Context` type are the key to how a runtime knows when to check any given future while still being lazy
- A type annotation for `self` works like type annotations for other function parameters but with two key differences:
    - It tells Rust what type `self` must be for the method to be called
    - It can't just be any type, it's restricted to the type on which the method is implemented, a reference or smart pointer to that type, or a `Pin` wrapping a reference to that type
- To poll a future to check whether it is `Pending` or `Ready(Output)`, need a `Pin` wrapped mutable reference to the type
- `Pin` is a wrapper for the pointer-like types such as `&`, `&mut`, `Box`, `Rc`
- `Pin` works with types that implement the `Deref` or `DerefMut` traits, but this is effectively equivalent to working only with pointers, `Pin` is not a pointer itself and does not have any behavior of its own like `Rc` and `Arc` do with reference counting, its purely a tool the compiler can use to enforce constraints on pointer usage
- `await` is implemented in terms of calls to `poll`, but that was in terms of `Unpin` not `Pin`, how does `Pin` relate to `Unpin`, and why does `Future` need `self` to be in a `Pin` type to call `poll`?
- Series of await points in a future get compiled into a state machine, and the compiler makes sure that the state machine follows all of `Rust's normal rules around safety, including borrowing and ownership, to make that work, Rust looks at what data is needed between one await point and either the next await point or the end of the async block, it then creates a corresponding variant in the compiled state machine, each variant gets the access it needs to the data that will be used in that section of the source code, whether by taking ownership of that data or by getting a mutable or immutable reference to it
- If anything is wrong about the ownership of references in a given async block, the borrow checker will indicate this, to move around the future that corresponds to that block, such as moving it into a `Vec` to pass to `join_all` will complicate things
- When moving a future (whether pushing it into a data structure to use as an iterator with `join_all` or returning it from what function), that actually means moving the state machine Rust creates, and unlike most other types in Rust, the futures Rust creates for async blocks can end up with references to themselves in the fields of any given variant
- By default, any object that has a reference to itself is unsafe to move, because references always point to the actual memory address of whatever they refer to, if the data strucutre itself is moved, those internal references will be left pointing to the old location, however, that memory location is now invalid, its value will not be updated when making changes to the data structure, the computer is now free to reuse that memory for other purposes, coudl end up reading completely unrelated data later
- The Rust compiler could try to update every reference to an object whether it gets moved, but that could add a lot of performance overhead, especially if a whole web of references needs updating, if the data structure in question could be made not to move in memory, wouldn't have to update any references, this is what Rust's borrow checker requires: in safe code, it prevents moving any item with an active reference to it
- `Pin` builds on that to give the exact guarantee needed, when pinning a value by wrapping a pointer to that value in `Pin`, it can no longer move, with `Pin<Box<SomeType>>`, `SomeType` is pinned, not the `Box` pointer
- The `Box` pointer can still move around freely, only care about making sure the data ultimately being referenced stays in place, if a pointer moves around, but the data it points to is in the same place, there is no problem
- Most types are safe to move around, even if they are behind a `Pin` wrapper, only need to think about pinning with internal references, primitive values such as numbers or booleans are safe because they don't have any internal references, neither do most types in Rust, can move around a `Vec` without worrying, if there is a `Pin<Vec<String>>`, would have to do everything via the safe but restricive APIs provided by `Pin` even though `Vec<String>` is always safe to move if there are no other references to it, need a way to tell the compiler that it's fine to move items around in cases like this, which is where `Unpin` comes in
- `Unpin` is a marker trait, similar to `Send` and `Sync` traits and has no functionality of its own, marker traits exist only to tell the compiler it's safe to use the type implementing a given trait in a particular context, `Unpin` informs the compiler that a given type does not need to uphold any guarnatees about whether the value in question can be safely moved
- Just as with `Send` and `Sync`, the compiler implements `Unpin` automatically for all types where it can prove it is safe, a special case (similar to `Send` and `Sync`) is where `Unpin` is not implemented for a type, the notation for htis is `impl !Unpin for SomeType` where `SomeType` is the name of a type that does not need to uphold those guarnatees to be safe whenever a pointer to that type is used in a `Pin`
- In other words, there are two things to keep in mind about the relationship between `Pin` and `Unpin`, first, `Unpin` is the normal case and `!Unpin` is the special case, whether a type implements `Unpin` or `!Unpin` only matters when using a pinned pointer to that type like `Pin<&mut SomeType>`
- With a `String`, it has a length and the Unicode characters that make it up, can wrap a `String` in `Pin`, however, `String` automtically implements `Unpin` as do most other types in Rust, as a result, can do things that would be illegal if `String` implemented `!Unpin` instead, such as replacing one string with another at the exact same locagtion in memory, this doesn't violate the `Pin` contract because `String` has no internal references that make it unsafe to move around which is why it implements `Unpin` rahter than `!Unpin`
- Originally tried ot make the futures produced by async blocks into a `Vec<Box<dyn Future<Output = ()>>>`, but those futures may have internal references so they don't implement `Unpin`, they need to be pinned and then can pass the `Pin` type into the `Vec`, confident that the underlying data in the futures will not be moved
- `Pin` and `Unpin` are mostly important for building lower-level libraries, or when building a runtime itself, rather than for day-to-day things
- This combination of `Pin` and `Unpin` makes it possible to safely implement a whole class of complex types in Rust that would otherwise prove challenging because they're self-referential, types that require `Pin` show up most commonly in async Rust today, but occaisonally in other contexts too
- The specifics of how `Pin` and `Unpin` work and the rules they're required to uphold are covered extensibely in the API documentation for `std::pin`

### The `Stream` Trait
- Streams are similar to asynchronous iterators, unlike `Iterator` and `Future`, `Stream` has no definition in the standard library, but there is a very common definition from the `futures` crate used throughouht the ecosystem
- With definitions of the `Iterator` and the `Future` traits before looking at how a `Stream` trait might merge them together, from `Iterator`, have the idea of a sequence, its `next` method provides an `Option<Self::Item>`, from `Future`, have the idea of readiness over time: its `poll` method provides a `Poll<Self::Output>`, to represent a sequence of items that become ready over time, can define a `Stream` trait that puts those features together
- Example: ```
use std::pin::Pin;
use std::task{Context, Poll};

trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>>;
}```
- The `Stream` trait defines an associated type called `Item` for the type of items produced by the stream, this is similar to `Iterator` where there may be zero to many items, and unlike `Future`, where there is always a singel `Output`, even if it's the unit type `()`
- Stream also defines a method to get those items, called `poll_next`, to make it clear that it polls in the same way `Future::poll` does and produces a sequence of items in teh same way `Iterator::next` does, its return type combines `Poll` with `Option`, the outer type is `Poll` because it has to be checked for readiness, just as a future does, the inner type is `Option` because it needs to signal whether there are more messages, just as an iterator does
- Something similar will likely be added to standard library, right now is in most runtimes
- In example in section on streaming, didn't use `poll_next` or `Stream`, but instead used `next` and `StreamExt`, could work directly in terms of the `poll_next` API by hand-writing custom `Stream` state machines and could work with futures directly via their `poll` method, using `await` is much nicer though, and the `StreamExt` trait supplies the `next` method to do that
- Example: ```
trait StreamExt {
    async fn next(&mut self) -> Option<Self::Item>
    where 
        Self: Unpin;
}```
- The actual definition used earlier looks different because it supports versions of Rust that did not support async functions in traits, as a result, it looked like `fn next(&mut self) -> Next<'_, Self> where Self: Unpin;`, the `Next` type is a `struct` what implements `Future` and allows naming the lifetime of the reference to `self` with `Next<'_, Self>` so that `await` can work with this method
- The `StreamExt` trait is also the home of all the interesting methods to use with Streams, `StreamExt` is automatically implemented for every type that implements `Stream` but these traits are defined separately to enable the community to iterate on convenience APIs without affecting the foundational trait, in the version of `StreamExt` used in the `trpl` crate, the trait not only defines the `next` method but also supplies a default implementation of `next` that correctly handles the details of caliling `Stream::poll_next`, this means that even when needing to write a custom streaming data type, only have to implement `Stream` and then oether users of the data type can use `StreamExt` and its methods with it automatically 

## Putting It All Together: Futures, Tasks, and Threads
- Threads provide one approach to concurrency, can also use async with futures and streams, in many cases instead of choosing one or the other can use both threads and async
- Many OS have supplied threading-based concurrency models and many languages support them as a result, models still have tradeoffs, on many OS, they use a fair bit of memory per thread and have some overhead for startup and shutdown, threads are also only an option when OS and hardware support them, unlike mainstream desktop and mobile computers, some embedded systems don't have an OS at all, so no threads
- The async model provides a different (and complementary) set of tradeoffs, in the async model, concurrent operations don't require their own threads, they can run on tasks as when `trpl::spawn_task` was used to kick off work from a synchronous function in the streams section, a task is similar to a thread but instead of being managed by OS is managed by library-level code: the runtime
- Can build a stream by using an async channel and spawning an async task that can be called from synchronous code, can do the same thing with a thread, previously used `trpl::spawn_task` and `trpl::sleep`, can replace these with `thread::spawn` and `thread:sleep` APIs from the standard library in the `get_intervals` function
- Example: ```
use std::pin::pin;
use std::time::Duration;
use std::thread;
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let intervals = get_intervals()
            .throttle(Duration::from_millis(100))
            .take(10);

        let mut stream = pin!(intervals);

        while let Some(count) = stream.next().await {
            println!("count: {count}");
        }
    });
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    thread::spawn(move || {
        let mut count = 0;
        loop {
            thread::sleep(Duration::from_millis(1));
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("error sending {count} {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}```
- Despite one of the functions spawning an async task on the runtime and the other spawning an OS thread, the resulting streams were unaffected by the differences
- Despite the similarities between spawning an async task and using an OS thread, the two approaches are quite different, can spawn millions of async tasks but doing same with threads would run out of memory
- There is a reason the two APIs are so similar: threads act as a boundary for sets of synchronous operations; concurrency is possible between threads, tasks act as a boundary for sets of asynchronous operations, concurrency is possible both between and within tasks, because a task can switch between futures in its body, futures are Rust's most granular set of concurrency and each future may represent a tree of other futures, the runtime (its executor), manages tasks, and tasks manage futures, tasks are similar to lighteight, runtime-managed, threads with added capabilities that come from being managed by a runtime instead of by the operating system
- This doesn't mean that async tasks are always better than threads, concurrency with threads is in some ways a simpler programming model than concurrency with `async`, that can be a strength or weakness, threads are somewhat fire and forget, they have no native equivalent to a future, so they simply run to completion without being interruptedexcept by the operating system itself, there is no built-in support for intratask concurrency the way futures do, threads in Rust also have no mechanisms for cancellation, whenever a future is ended, its state is cleaned up correctly
- These limitations also make threads harder to compose than futures, it's much more difficult to use threads to build helpers such as the `timeout` and `throttle` methods built earlier, the fact that futures are richer data structures mean they can be composed together more naturally
- Tasks give additional control over futures, allowing the selection of where and how to group them, threads and tasks often work very well together because tasks can (at least in some runtimes) be moved around between threads, in fact, under the hood, the runtime in use (including the `spawn_blocking` and `spawn_task` functions) is multithreaded by default, many runtimes use an approach called work stealing to transparently move taks around between threads, based on how the threads are currently being utilized, to improve the system's overall performance, this approach actually requires threads and tasks, and therefore futures
- When considering which method to use consider this:
    - If the work is parallelizable, such as processing a bunch of data where each part can be processed separately, threads are a better choice
    - If the work is very concurrent, such as handling messages from a bunch of different sources that may come in at different intervals or different rates, async is a better choice
- If both parallelism and concurrency are needed, don't have to choose between threads and async, can use them together freely, letting each one play the part it's best at, for example:
- Example: ```
fn main() {
    let (tx, mut rx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    trpl::run(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
}```
- Begin by creating an async channel, then spawn a thread that takes ownership of the sender side of the channel, within the thread, can send the numbers 1 through 10, sleeping for a second between each, finally run a future created with an async block passed to `trpl::run`, in the future, those messages are awaited
- To return to the original scenario, imagine running a set of video encoding tasks using a dedicated thread (since video encoding is compute-bound) but notifying the UI that those operations are done with an async channel
