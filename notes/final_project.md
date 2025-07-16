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
- The first arm is the same as the `if` block from earlier and checks if the URI is / to respond with hello.html, the second arm matches a request to /sleep, when the request is received, teh server will sleep for five seconds before rendering a successful HTML page, the third arm is the same as the `else` block from earlier that checks whether none of the previous cases have been met
- This is quite primitive, real libraries would handle the recognition of multiple requests in a much less verbose way
- When querying /sleep and then loading /, / will wait until `sleep` has slept for its full five seconds before loading
- There are multiple techniques to avoid requests backing up behind a slow request, including using async, the one to implement here is a thread pool

### Improving Throughput with a Thread Pool
- A thread pool is a group of spawned threads that are waiting and ready to handle a task, when the program receives a new task, it assigns one of the threads in the pool in to the task, and that thread will process the task
- 
