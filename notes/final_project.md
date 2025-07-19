# Final Project: Building a Multithreaded Web Server
- For final project, will make a web server that says "hello"
- Plan for building the web server:
    1. Learn a bit about TCP and HTTP
    2. Listen for TCP connections on a socket
    3. Parse a small number of HTTP requests
    4. Improve the throughput of the server with a thread pool
- First method won't be best way to build a web server with Rust, there are already production-ready crates on crates.io that provide more complete web server and thread pool implementations
- Since Rust is a systems programming language, can choose the level of abstraction to work with and go to a lower level than is possible or practical in other languages
- Will not be using async and await, will note how async and await might be applicable to some of the same problems in here, many async runtimes use thread pools for managing their work
- Will write the basic HTTP server and thread pool manually

## Building a Single-Threaded Web Server
- Will start by getting a single-threaded web server working, before beginning, will look at protocols involved with building web servers
- The two main protocols involved with web servers are Hypertext Transfer Protocol (HTTP) and Transmission Control Protocol (TCP)
- Both protocls are request-response protocols, meaning a client initiates requests and a server listens to the requests and provides a response to the client
- The contents of those requests and responses are defined by the protocol
- TCP is the lower-level protocl that describes the details of how information gets from one server to another but doesn't specify what the information looks like, HTTP builds on top of TCP by defining the contents of the requests and responses, it's technically possible to use HTTP with other protocols, but in the vast majority of cases, HTTP sends its data over TCP, will work with raw bytes of TCP

### Listening to the TCP connection
- Web server needs to listen to a TCP connection, first part to do, the standard library offers a `std::net` module that facilitates this
- This code will listen at the local address `127.0.0.1:7878` for incoming TCP streams, when it gets an incoming stream, it will print  `Connection established`
- Example: ```
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        println!("connection established");
    }
}```
- Using `TcpListener`, can listen for TCP connecetions at the address `127.0.0.1:7878`, in the address, the section before the colon is an IP address representign the computer (same on every computer), `7878` is the port, have chosen this port for two reasons: HTTP isn't normally accessed on this port so the server is unlikely to conflict with any other web server on this machine, and 7878 is rust typed on a telephone
- The `bind` function in this secnario works like the `new` function in that it will return a new `TcpListener` instance, the function is called `bind` because, in networking, connecting a port to listen is known as "binding to a port"
- The `bind` function returns a `Result<T, E>`, which indicates that it's possible for binding to fail, for example, connecting to port 80 requires administrator privileges (non-administrators can only listen to ports higher than 1023) if trying to connect to port 80 without being an administrator, this wouldn't work, binding also wouldn't work, binding also woulnd't work if running two sintances of the program and two programs listening to same port, won't worry about these kinds of errors here, will just use `unwrap` to stop this program if errors happen
- The `incoming` method on `TcpListener` returns an iterator that gives a sequence of streams (streams of type `TcpStream`)
- A single stream represetns an open connection between the client and the server, a connection is the name for the full request and response process in which a client connects to the server, the server generates a response, and the server closes the connection, as such, will ready from the `TcpStream` to see what the client sent and then write response in the stream to send dat aback to the client, this `for` loop will process each connection in turn and produce a series of streams to handle
- For now, handling of the stream consists of calling `unwrap` to terminate the program if the stream has any errors, if there aren't any errors, the program prints a message, will add functionality for the success case in the next listing, the reason errors may be received from the `incoming` method when a client conncets to the server is that this isn't actually iterating over connections, instead, it iterates over connection attempts, the connection might not be successful for a number of reasons, many of them are operating system specific, for example, many operating systems limit the number of simultaneous connections, new connection attempts beyond this limit produce an error until some of the open connections are closedG
- Browser will show error like "connection reset" when running this because the server currently doesn't send any data back but there are multiple connections established
- Will sometimes see multiple messages printed for one browser request, the reason might be that the browser is making a request for the page as well as a request for other resources, like the favicon.ico that appears in the browser tab
- Could also be that the browser is trying to connect to the server multiple times because the server isn't responding with any data
- When `stream` goes out of scope and is dropped at the end of the loop, the connection is closed as part of the `drop` implementation, browsers somtimes deal with closed connections by trying because the problem may be temporary
- Browsers also open multiple connections to the server without sending any requests, so that if they do later send requests, they can happen faster, when this happens, the server will see each connection, regardless of whether there are requests over that connection, many versions of Chrome-based browsers do this, for example, can disable that optimization by using private browsing mode or a different browser
- Have successfully gotten a handle to a TCP connection
- Need to stop program by pressing `ctrl`-`c` when done with particular version of the code, then restarting the program by invoking the `cargo run` commad after making changes to the code

### Reading the Request
- Need function to get connection and take some action with the connection, `handle_connection` to read data from the TCP stream and print it to see the data being sent from the browser
- Example: ```
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:#?}", http_request);
}```
- Have brought `std::io::prelude` and `std::io::BufReader` into scope to get access to traits and types to read and write to the stream in the `for` loop in the `main` function instead of printing a message that says a connection has been established, now call the new `handle_connection` function and pass the `stream` to it
- In the `handle_connection` function, have created a new `BufReader` instance that wraps a reference to the `stream`, the `BufReader` adds buffering by managing calls to the `std::io::Read` trait methods
- Have created a variable `http_request` to collect the lines of the request the browser sends to the server, have indicated to collect these lines in a vector by adding a `Vec<_>` type annotation
- `BufReader` implements the `std::io::BufRead` trait which provides the `lines` method, which returns an iterator of `Result<String, std::io::Error>` by splitting the stream of data whenever it sees a newline byte
- To get each `String`, can map and `unwrap` each `Result`, the `Result` might be an error if the data isn't valid UTF-8 or if there was a problem reading from the stream, a production program should handle these errors more gracefully, choosing to stop the program in error case for simplicity
- The browser signals the end of na HTTP request by sending two newline characters in a row, so to get one request from the stream, can take lines until receiving a line that is the empty string, once collected the lines into the vector, they are printed using pretty debug formatting to review the instructions the web browser sends to the server
- Can figure out why this receives multiple connections by looking at the path after `GET` in the first line of the request, if the repeated connections are all requesting /, the browser is retrying to fetch / repeatedly because it's not getitng a response from the program

### A Closer Look at an HTTP Request
- HTTP is a text-based protocol, and a request takes this format:
```
Method Request-URI HTTP-Version CRLF
Headers CRLF
message-body```
- The first lien is the request line that holds information about what the client is requesting, the first part of the request line indicates the method used, such as `GET` or `POST`, which describes how the client is making this request, this client used a `GET` request, which means it is asking for information
- The next part of the request is /, indicating the uniform resource identifier (URI), the cient is requesting: a URI is almost, but not quite the same as a uniform resource locator (URL), HTTP spec uses term URI, can just mentally substitute URL for URI here
- The last part is the hTTP version the client uses, and then the request line ends in a CRLF sequence (CRLF stands for carriage return and line feed, terms from the typewriter days) the CRLF sequence can also be written as `\r\n`, where `\r` is a carriage return and `\n` is a line feed, the CRLF sequence separates the request line from the rest of the request data, when CRLF is printed there is a new line start rather than `\r\n`
- Looking at request line data received from running program so far, see that `GET` is the method, / is the request URI, and `HTTP/1.1` is the version
- After the request URI, the returning lines starting from `Host:` onward are headers, `GET` requests have no body

### Writing a Resposne
- Responses have the following format: ```
HTTP-Version Status-Code Request-Phrase CRL
headers CRLF
message-body```
- The first line is a status line that contains the HTTP version used in the response, a nueric status code that summarizes the result of the request, and a reason phrase that provides a text summarization of the status code, after the CRLF sequence are any headers, another CRLF sequence, and the body of the response
- Here is an example that uses HTTP version 1.1, and has a status code of 200, an OK reason phrase, no headers, and no body: `HTTP/1.1 200 OK\r\n\r\n`
- The status code 200 is the standard success response, the text is a tiny successful HTTP response, will write this to the stream as the response to a cuessful request, from the `handle_connection` function will remove the `println` that was printing the request data and replace it with this response
- The first new line defines the `response` variable that holds the success message's data, then called `as_bytes` on the response to convert the string data to bytes, the `write_all` method on `stream` takes a `&[u8]` and sends thoe bytes directly down the connection, since the `write_all` operation could fail, can use `unwrap` on any error result as before, would add error handling here in a real application

### Returning Real HTML
- Will implement functionality for returning more than a blank page, in file hello.html at root of project directory, can put any HTML
- To return it from the server, when a request is recevied, will modify `handle_connection` to read the HTML file, add it to the response as a body and send it
- Example: ```
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.0 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}```
- Have added `fs` to the `use` statement to bring the standard library's file system module into scope, the code for reading the contents of a file to a string uses the `read_to_string` method
- Next, have used `format!` to add the file's contents as the body of the success response, to ensure a valid HTTP response, have added the `Content-Length` header which is set to the size of the response using the `len` method, and in this case is the size of `hello.html`
- Currently, this ignores the request data in `http_request` and just sends back the contents of the HTML file unconditionally, that means if requesting something else like 127.0.0.1:7878/somethign-else in the browser, will still receive same HTML response, server is currently very limited and does not do what most web servers do, want to customize the responses depending on the request, and only send back the HTML file for a well-formed request to /

### Valditating the Request and Selectively Responding
- Right now, web server will return the HTML in the file no matter what the client requested, will add functionality to check the browser is requesting / before returning the HTML file and return an error if the browser requests anything else, for this need to modify `handle_connection`, this new code chekcs the content of the request recieved against what a request for / looks like and adds `if` and `else` blocks to treat request differently
- Example: ```
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap()!

    if request_line == "GET / HTTP/1.1\r\n" {
        let status_line = "HTTP/1.0 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {

    }
}```
- Only going to be looking at the first line of the HTTP request, rather than reading the entire request into a vector, calling `next` to get the first item from the iterator, the first `unwrap` takes care of the `Option` and stops the program if the iterator has no items, the second `unwrap` handles the `Result`, and has the same effect as the `unwrap` that was in the `map` from earlier
- Next, need to check if the `request_line` to see if it equals the request line of a GET request to the / path, if it does, the `if` block returns the contents of the HTML file
- If the `request_line` does not equal the GET request to the / path, it means have received some other request, will add code to the `else` block to respond to all other requests
- Running this code and requesting something other than / will result in a connection error
- Need to add code to the `else` block to return a resposne with the status code 404, which signals that the content for the request was not found, will also return some HTML for a page to render in the browser to the end user 
- Example: ```
    } else {
        let status_line = "HTTP/1.0 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }```
- Here, the response has a status line with the status code 404 and the reason phrase `NOT FOUND`, the body of the response will be the hTML in the file 404.html, will need to create a 404.html file next to the hello.html for the error page
- With these changes, server should return the contents of hello.html and any othe request should return the error HTML from 404.html

### A Touch of Refactoring
- At the moment, the `if` and `else` blocks have a lot of repetition, they're both reading files and writing the contents of the files to the stream
- Only difference are the status line and filename, can make the code more concise by moving those differences into separate `if` and `else` lines that will assign the values of the status line and the filename to variables
- Example: ```
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.0 200 OK", "hello.html")
    } else {
        ("HTTP/1.0 404 NOT FOUND", "404.html")
    }

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}```
- Now the `if` and `else` blocks return the appropriate values for the status line and filename in a tuple, then use destructuring to assign these two values to `status_line` and `filename` using a patter in the `let` statement
- The previously duplicated code is now outside the `if` and `else` blocks and uses the `status_line` and `filename` variables, this makes it easier to see the difference between the two cases, and it means there is only one place to update hte code to change how the file reading and response writing work
- Currently server runs in a single thread, meaning it can only serve one request at a tine, now will try to fix server to handline multiple requests at once

## Turning the Single-Threaded Server into a Multithreaded Server
- Right now, the server will process each request in turn, meaning it won't process a second connection until the first is finished processing, if the server received more and more requests, this serial execution would be less and less optimal if the server receives a request that takes a while to process, subsequent requests will be blocked by the current request and have to wait, even if the new requests are processed quickly, need to fix this

### Simulating a Slow Request in the Current Server Implementation
- Will look at how a slow-processing request can affect other requests made to the current server implementation
- This will handle a request to /sleep with a simulated slow response that will cause the server to sleep for five seconds before responding
- Example: ```
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}```
- This switches from `if` to `match` now that there are three cases, need to explicitly match on a slice of `request_line` to pattern match against the string literal values: `match` doesn't do automatic referencing and dereferencing like the equality method does
- The first arm is the same as the `if` block from earlier and checks if the URI is / to respond with hello.html, the second arm matches a request to /sleep, when the request is received, the server will sleep for five seconds before rendering a successful HTML page, the third arm is the same as the `else` block from earlier that checks whether none of the previous cases have been met
- This is quite primitive, real libraries would handle the recognition of multiple requests in a much less verbose way
- When querying /sleep and then loading /, / will wait until `sleep` has slept for its full five seconds before loading
- There are multiple techniques to avoid requests backing up behind a slow request, including using async, the one to implement here is a thread pool

### Improving Throughput with a Thread Pool
- A thread pool is a group of spawned threads that are waiting and ready to handle a task, when the program receives a new task, it assigns one of the threads in the pool in to the task, and that thread will process the task
- The remaining threads in the pool are available to handle any other tasks that come in while the first thread is processing, when the first thread is done processing its task, it's returned to the pool of idle threads, ready to handle a new task, a thread pool allows processing of connections concurrently, increasing the throughput of a server
- Will limit the number of threads in the pool to a small number to protect from DoS attacks, if program created a new thread for each request as it came in, making millions of requests to a server could created havoc by using up all server's resources and grdining the processing of requests to a halt
- Instead of spawning unlimited threads, will have a fixed number of threads in the pool
- Requets that come in are sent to the pool for processing, the pool will maintain a queue of incoming requests, each of the threads in the pool will pop off a request from this queue, handle the request, then ask the queue for another request, with this design, can process up to `*N*` requests concurrently, where `*N*` is the number of threads, if each thread is responding to a long-running request, subsequent requests can still back up in the queue, but have increased the number of long-running requests to handle before reaching that point
- This technique is one of many ways to improve the throughput of a web server, other options to explore are the fork/join model, the single-threaded async I/O model, and the multithreaded async I/O model
- Before begining to implement the thread pool, should determine the structure of the pool, when designing code, writing the client interface first can help guide the design, writing the API of the code so it's structured in the way it should be called, then implementing functionality within that structure rather than implementing the functionality the public API
- Similar to using TDD earlier, will use compiler-driven development here, will write code that calls the functions intended, then will look at errors from the compiler to determine what to change to get the code to work, before doing that, will explore the technique not to use as a starting point

#### Spawning a Thread for Each Request
- First, will explore how code may look if it did create a new thread fo revery connection, this isn't the final plan due to the problems with potentially spawning an unlimited number of threads, but it is a starting point to get a working multithreaded server first, will add the thread pool as an improvement, and contrasting the two solutions will be easier
- Will make changes to `main` to spawn a ne thread to handle each stream within the `for` loop
- Example: ```
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}```
- `thread::spawn` will create a new thread, and then run the code in the closure in the new thread, if running this code and then /sleep in the browser, then other more / in other tabs, will see that the requests don't need to wait for /sleep to finish, but htis will eventually overwhelm the system since new threads would be created without any limit, this is the kind of situation where async and await are valuable

#### Creating a Finite Number of Threads
- Wnat the thread pool to work in a similar, familiar way, so that switching from threads to a thread pool doesn't require large changes to the code that uses the API, here is a hypothetical interface for a `ThreadPool` structto use instead of `thread::spawn`
- Example: ```
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}```
- Used `ThreadPool::new1` to create a new thread pool with a configurable number of threads, in this case, four, then, in the `for` loop, `pool.execute` has a similar interface as `thread::spawn` in that it takes a closure in the pool should run for each stream, need to implement `pool.execute` so it takes the closure and gives it to a thread in the pool to run, this code won't compile yet, but will try so the compiler can guide in how to fix it

#### Building `ThreadPool` Using Compiler Driven Development
- After making the previous changes to src/main.rs, and running compiler from `cargo check`, get an error stating that `ThreadPool` type or module is needed so need to build one, will switch the `hello` crate from a binary crate to a library crate to hold the `ThreadPool` implementation, after changing to a library crate, can also use the separate thread pool library for any work to do using a thread pool, not just serving web requests
- Will create a src/lib.rs file that contains a simple definition of a `ThreadPool` struct: `pub struct ThreadPool`
- Then bringing `ThreadPool` into scope from the library craye by adding the following code to the top of `src/main.rs`: `use hello::ThreadPool;`
- Code still does not work, error states that need to add associated function `new` for `ThreadPool`
- Also know that `new` needs to have one parameter that can accept `4` as an argument and should return a `ThreadPool` instance, will implement the simplest `new` function that will have those characteristics
- Example: ```
pub struct ThreadPool;

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        ThreadPool
    }
}```
- Have chosen `usize` as the type of the `size` parameter because a negative number of threads doesn't make any sense, also know to use this `4` as the number of elements in a collection of threads, which is what the `usize` type is for
- Now, current error is that the `execute` method is missing on `ThreadPool`, have decided current thread pool should have an interface similar to `thread::spawn`, in addition, will implement the `execute` function so it takes the closure it's given and gives it to an idle thread in the pool to run
- Will define the execute method on `ThreadPool` to take a closure as a parameter, closures can be taken as parameters with three different traits: `Fn`, `FnMut`, and `FnOnce`, need to decide which kind of closure to use here, will end up doing something similar to the standard library `thread::spawn` implementation, can look at what bounds the signature of `threads::spawn` has on its parameter, the documentation shows the following: ```
pub fn spawn<F, T>(f: f) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,```
- The `F` type parameter is important here, the `T` type parameter is related to the return value, can see that `spawn` uses `FnOnce` as the trait bound on `F`, this is likely what is wanted as well because will eventually pass the argument obtained in `execute` to `spawn`
    - Also using `FnOnce` as the trait to use because the thread for running a request will only execute that request's closure one time, which matches the `Once` in `FnOnce`
- The `F` parameter also has the trait bound `Send` and the lifetime bound `'static`, which are useful in this situation, need to transfer the closure from one thread to another and `'static` because it is unknown how long the thread will take to execute, will create an `execute` method on `ThreadPool` that will take a generic parameter of type `F` with these bounds: ```
impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where:
        F: FnOnce() + Send + 'static,
    {}
}```
- Will stil use the `()` after `FnOnce` because this `FnOnce` represents a closure that takes no parameters and returns the unit type `()` just like function definitions, the return type can be omitted from the signature, but even if there are no parameters, will still need the parentheses
- Current code compiles, if running `cargo run` and making request, will not have correct behavior, library isn't actually calling the closure passed to `execute` yet
- If this were a real, complete, project, this would be a good time to start writing unit tests to check that the code compiles and has the correct behavior

#### Valdiating the Number of Threads in `new`
- Not doing anything with the parameters to `new` and `execute`, need to implement the bodies of these functions with the desired behavior
    - With `new`, chose an unsigned type for the size parameter since a pool with a negative number of threads makes no sense, a pool with zero threads also makes no sense, but zero is a valid `usize`, need to add code to check that `size` is greater than zero before returning a `ThreadPool` instance and having the program panic if it receives a zero using the `assert!` macro
- Example: ```
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }```
- Have also added some documentation for `ThreadPool` with doc comments
- Have followed good documentation practices by adding a section that calls out the situations in which the function can panic, can view docs using `cargo doc --open`
- Instead of adding the `assert!` macro as done here, could change the `new` into `build` and return a `Result` as done earlier with `Config::build` in I/O project, but have decided that trying to create a thread pool without any threads should be an unrecoverable error
- A function named build would be similar to the following: `pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {}`

#### Creating a Space to Store the Threads
- Now that there is a way to know there is a valid number of threads to store in the pool, can create those threads and store them in the `ThreadPool` struct before returning the struct
- `thread::spawn` function returns a `JoinHandle<T>`, where `T` is the type that the closure returns, in this case the closures passed to the thread pool will handle the connection and not return anything, so `T` will just be the unit type `()`
- The following code will compile but won't create any threads yet, have changed the definition of `ThreadPool` to hold a vector of `thread::JoinHandle<()>` instances, initialized the vector with a capacity of size, set up a `for` loop that will run some code to create the threads and returned a `ThreadPool` instance containing them
- Example: ```
use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create threads and store them in the vector
        }

        ThreadPool { threads }
    }```
- Have brought `std::thread` into scope in the library crate becuase this uses `thread::JoinHandle` as the type of the items in the `ThreadPool`, once a valid size is received, the `ThreadPool` creates a new vector that can hold `size` items, the `with_capacity` function performs the same task as `Vec::new` but pre-allocates space in the vector, since it is known that `size` elements need to be stored in the vector, this doing this allocation up front is more efficient than using `Vec::new`, which resizes itself as elements are inserted

#### A `Worker` Struct Responsible for Sending Code from the `ThreadPool` to a `Thread`
- Left a comment in the `for` loop regarding the creation of threads, will look at how to actually create threads, the standard library provides `thread::spawn` as a way to create threads, and `thread::spawn` expects to get some code the thread should run as soon as the thread is created, however, in this case, want to create the threads and have them wait for code that will be sent later, the standard library's implementation of threads doesn't hinclude any way to do this and will implement this manually
- Will implement this behavior by introducing a new data structure between the `ThreadPool` and the threads that will manage this new behavior, will call this data structure Worker, which is a common term in pooling implementations, the `Worker` picks up code that needs to be run and runs the code in the Worker's thread
- The workers wait until orders come in from customers, and then they're responsible for taking those orders and fulfilling them
- Instead of storng a vector of `JoinHandle<()>` instances in the thread pool, will store instances of the `Worker` struct, each `Worker` will store a single `JoinHandle<()>` instance, then will implement a method on `Woker` that will take a closure of code to run and send it to the already running thread for execution, will also give each `Worker` and `id` to distinguish between the different instances of `Worker` in the pool when logging or debugging
- Here is the new process that will happen when creating a `ThreadPool`, will implement the code that sends the closure to the thread after `Worker` is set up in this way:
    1. Define a `Worker` struct that holds an `id` and a `JoinHandle<()>`
    2. Change `ThreadPool` to hold a vector of `Worker` instances
    3. Define a `Worker::new` function that takes an `id` number and returns a `Worker` instance that holds the `id` and a thread spawned with an empty closure
    4. In `ThreadPool::new`, will use the `for` loop counter to generate an `id`, create a new `Worker` with that `id`, and store the worker in the vector
- Example: ```
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create workers and store them in the vector
            // workers.push(worker);
            workers.push(Worker::new(i));
        }

        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // 1. select worker from pool queue
        // 2. run task
        // 3. add new worker to queue?
        // alternatively could find way to borrow thread and run task
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}```
- Have changed the name of the field on `ThreadPool` from `threads` to `workers` because it's now holding `Worker` instances insteaf of `JoinHandle<()>`, have used the counter in the `for` loop as an argument to `Worker::new`, and stored each new `Worker` in the vector named `workers`
- External code such as the server in src/main.rs don't need to know the implementation details regarding a worker struct within `ThreadPool`, so `Worker` struct and its `new` function are private, the `Worker::new` function uses the `id` given and stores a `JoinHandle<()>` instance created by spawning an empty closure
- Note: if the operating system can't create a thread because there aren't enough system resources, `thread::spawn` will panic which will cause the whole server to panic, even though the creation of some threads might succeed, for simplicity's sake, this behavior is fine, but in a production thread pool implementation, may want to use `std::thread::Builder` and its `spawn` method that returns `Result` instead
- This code will compile and will store the number of `Worker` instances specified as an argument to `ThreadPool::new` but this still isn't processing the closure obtained in `execute`, will look at how to do that next

#### Sending Requests to Threads via Channels
- Next problem is that the closures given to `thread::spawn` do nothing, currently, get the closure to execute in the `execute` method, but then, need to give `thread::spawn` a closure to run when creating each `Worker` during the creation of the `ThreadPool`
- Want the `worker` structs that were created to fetch the code to run from a queue held in the `ThreadPool` and send that code to its thread to run
- Channels are a simple way to communicate between two threads, good for this use case, can ue a channel to function as the queue of jobs, and `execute` will send a job from the `ThreadPool` to the `Worker` instances, which will send the job to its thread, here is the plan:
    1. The `ThreadPool` will create a channel and hold on to the sender
    2. Each `Worker` will hold on to the receiver
    3. Will create a new `Job` struct that will hold the closures to send down the channel
    4. The `execute` method will send the job it wants to execute through the sender
    5. In its thread, the `Worker` will loop over its receiver and execute the closures of any jobs it receives
- Will start by creating a channel in `ThreadPool::new` and holding the sender in the `ThreadPool`, the `Job` struct doesn't hold anything for now but will be the type of the item to send down the channel
- Example: ```
use std::{sync::mpsc, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create workers and store them in the vector
            // workers.push(worker);
            workers.push(Worker::new(id));
        }

        ThreadPool { workers, sender }
    }```
- Here, have modified `ThreadPool` to store the sender of a channel that transmits `Job` instances
- In `ThreadPool::new`, have created a new channel and have the pool hold the sender, this will compile
- Will try passing a receiver of the channel into each `Worker` as the thread pool creates the channel, want to use the receiver in the thread that the `Worker` instances spawn to reference the `receiver` parameter in the closure, this won't compile yet
- Example: ```
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create workers and store them in the vector
            // workers.push(worker);
            workers.push(Worker::new(id, receiver));
        }

        ThreadPool { workers, sender }
    }

    impl Worker {
        fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
            let thread = thread::spawn(|| {
                receiver;
            });

            Worker { id, thread }
        }
    }```
- Have made small and straightforward changes, passed the receiver into `Worker::new` and then used it inside the closure
- This code results in an error, code is trying to pass `receiver` to multiple `Worker` instances, this won't work because the channel implementation is multiple producer and single consumer, means can't just clone the consuming end of the channel to fix this, also don't want to send a message multiple times to multiple consumers, want one list of messages with multiple `Worker` instances such that each message gets processed once
- Additionally, taking a job off the channel queue involves mutating the `receiver`, so the threads need a safe way to share and modify `receiver`, otherwise, might get race conditions
- Can use `Arc`, thread-safe smart pointers to share ownership across multiple threads and allow the threads to mutate the value, `Arc<Mutex<T>>` can be used
    - The `Arc` type will let multiple `Worker` instances own the receiver, and `Mutex` will ensure that only one `Worker` gets a job from the receiver at a time
- Example: ```
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(|| {
                receiver;
            });

            Worker { id, thread }
        }
    }```
- In `ThreadPool::new`, have put the receiver in an `Arc` and a `Mutex`, for each new `Worker`, clone the `Arc` to bump the reference count so the `Worker` instances can share ownership of the receiver

#### Implementing the `execute` Method
- Will implement the `execute` method on `ThreadPool`, will also change `Job` from a struct to a type alias for a trait object that holds the type of closure that `execute` receives, type aliases allow making long types shorter for ease of use
- Example: ```
    type Job = Box<dyn FnOnce() + Send + 'static>;

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }```
- Creating a `Job` type alias for a `Box` that holds each closure and then sending the job down the channel
- After creating a new `Job` instance using the closure obtained in `execute`, will send that job down the sending end of the channel, have called `unwrap` on `send` for the case that sending fails, this might happen if, for example, all threads are stopped from executing, meaning the receiving end has stopped reading new messages, at the moment, can't stop threads from executing: threads continue executing as long as the pool exists, the reason `unwrap` is used is that the failure case won't happen but compiler doesn't know this
- In the `Worker`, the closure passed to `thread::spawn` still only references the receiving end of the channel, instead, need the closure to loop forever, asking the receiving end of the channel for a job and running the job when it gets one, will make the change to `Worker::new`
- Example: ```
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {id} got job, executing...");

                job();
            }
        });

        Worker { id, thread }
    }
}```
- Here, first call `lock` on the `receiver` to acquire the mutex and then call `unwrap` to panic on any errors, acquiring a lock might fail if the mutex is in a poisoned state, which can happen if some other thread panicked while holding the lock, rather than releasing the lock, in this situation, calling `unwrap` to have this thread panic is the correct action to take, could change `unwrap` to an `expect` with an error message that is contextually meaningful
- If the lock is acquired on the mutex, can call `recv` to receive a `Job` from the channel, a final `unwrap` moves past any errors here as well, which might occur if the thread holding the sender has shut down, similar to how the `send` method returns `Err` if the receiver shuts down
- The call to `recv` blocks so if there is no job yet, the current thread will wait for a job to become available, the `Mutex<T>` ensures that only one `Worker` thread at a time is trying to request a job
- Thread pool is now in a working state, now have a thread pool that executes asynchronously, there are never more than four threads created, so the system won't get overloaded if the server receives a lot of requests, if a request is made to /sleep, the server will be able to serve other requests by having another thread run them
- If opening /sleep in multiple browser windows simultaneously, they might load one at a time in five-second intervals, some web browsers execute multiple instances of the same request sequentially for caching reasons, this limitation is not caused by the web server
- `while let` loop was not used for following reasons:
- Code with `while let` loop compiles but doesn't run desired threading behavior, a slow request will still cause other requests to wait to be processed, the reason is the following: the `Mutex` struct has no public `unlock` method because the ownership of the lock is based on the lifetime of `MutexGuard<T>` within the `LockResult<MutexGuard<T>>`, at compile time, the borrow checker can then enforce the rule that a resource guarded by a `Mutex` cannot be accessed unless holding that lock, however, this implementatin can also result in the lock being held longer than intended if not mindful of the lifetime of the `MutexGuard<T>`
- `let job = receiver.lock().unwrap().recv().unwrap();` works because with `let`, any temporary values used in the expression on the right hand side of the equal sign are immediately dropped then the `let` statement ends, however, `while let` and `if let` and `match` don't drop temporary values until the end of the associated block, with a `while let` loop, the lock remains held for the duration of the call to `job()`, meaning other `Worker` instances cannot receive jobs

## Graceful Shutdown and Cleanup
- Previous code is responding to requests asynchronously through the use of a thread pool as intended, receieve some warnings about the `workers`, `id`, and `thread` fields that are not used in a direct way, indicates there is no cleanup, when using `ctrl`-`c` on the main thread, all other threads are stopped immediately as well, even if they're in the middle of sending a request
- Next, will implement `Drop` trait to call `join` on each of the threads in the pool so they can finish the requests they're working on before closing, then implement a way to tell the threads they should stop accepting new requests and shut down, to see this in action, need to modify the server to accept only two requests before gracefully shutting down its thread pool
- One thing to notice, none of this affects the parts of the code that handle executing the closures, so everything here would just be the same if using a thread pool for an async runtime

### Implementing the `Drop` Trait on `ThreadPool`
- Will start with implementing `Drop` on the thread pool, when the pool is dropped, the threads should all join to make sure they finish their work
- Example: ```
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}```
- First this loops through each of the thread pool `workers`, use `&mut` for this because `self` is a mutable reference and also need to be able to mutate `worker`, for each worker, print a message saying that htis particular `Worker` instance is shutting down, then call `join` on that `Worker` instance's thread, if the call to `join` fails, use `unwrap` to make Rust panic and go into an ungraceful shutdown
- Receive an error during compilation that can't call `join` because only have a mutable borrow of each `worker` and `join` tkaes ownership of its argument, to solve this issue need to move the thread out of the `Worker` instance that owns `thread` so `join` can consume the thread, one way to do this is if `Worker` held an `Option<thread::JoinHandle<()>>`, could call the `take` method on the `Option` to move the value out of the `Some` variant and leave a `None` variant in its place, in other words, a `Worker` that is running would have a `Some` variant in `thread` and when wanting to clean up a worker, would replace `Some` with `None` so the `Worker` wouldn't have a thread to run
- However, the only time this could come up would be when dropping the `Worker`, in exchange, would need to deal with an `Option<thread::JoinHandle<()>>` anywhere accessed `worker.thread`, idiomatic Rust uses `Option` frequently, but when needing to wrap something, will always be present in `Option` as a workaround, should look for alternative approaches, they can make code cleaner and less error prone
- A better alternative exists: the `Vec::drain` method, it accepts a range parameter to specify which items to remove from the `Vec`, and returns an iterator of those items, passing the `..` range syntax will remove every value from the `Vec`
- Example: ```
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in self.workers.drain(..) {
            println!("Shutting down worker: {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}```
- This resolves the compiler error and does not require any other changes to code

### Signaling to the Threads to Stop Listening for Jobs
- With all the changes made, code compiles without any warning, but this code doesn't function as intended yt, the key is the logic in the closures run by the threads of the `Worker` instances, at the moment, `join` is caleld but that won't shut down the threads as they `loop` forever looking for jobs, if trying to drop the `ThreadPool` in the current implementation of `drop`, the main thread will block forever, waiting for the first thread to finish
- To fix this thread, will need a change in the `ThreadPool` drop implementation and then a change in the `Worker` loop
- Will first change the `ThreadPool` `drop` implementation to explicitly drop the `sender` before before waiting for the threads to finish, unlike with the thread here, do need to use an `Option` to be able to move `sender` out of `ThreadPool` with `Option::take`
- Example: ```
use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { 
            workers, 
            sender: Some(sender) ,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            println!("Shutting down worker: {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {id} got job, executing...");

                job();
            }
        });

        Worker { id, thread }
    }
}```
- Dropping `sender` closes the channel, which indicates that no more messages will be sent, when that happens, all the calls to `recv` that the `Worker` instances do in the infinite loop will return an error, need to change the `Worker` loop to gracefully exist the loop in that case, which means the threads will finish when the `THreadPool` `drop` implementation calls `join` on them
- Example: ```
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job, executing...");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down");
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}```
- To see this in action, will modify `main` to accept only two requests before gracefully shutting down the server
- Example: ```
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

   let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}```
- Wouldn't want a real-world web server to shut down after serving only two requests, this code just demonstrates that the graceful shutdown and cleanup is in working ordder
- The `take` method is defined on the `Iterator` trait and limits the iteration to the first two items at most, the `ThreadPool` will go out of scope at the end of `main`, and the `drop` implementation will run
- Server will stop accepting connections after the second connection, and the `Drop` implementation on `ThreadPool` starts executing before `Worker` 3 even starts its job, dropping the `sender` disconnects all the `Worker` instances and tells them to shut down, the `Worker` instances each print a message when they disconnect, and then the thread pool calls `join` to wait for each `Worker` thread to finish
