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
- For now, handling of the stream consists of calling `unwrap` to terminate the program if the stream has any errors, if there aren't any errors, the program prints a message, will add functionality for the success case in the next listing, the reason errors may be received from the `incoming` method when a client conncets to the server is that this isn't actually iterating over connections, instead, it iterates over connection attempts, the connection might not be successful for a number of reasons, many of them are operating system specific, for example, many operating systems limit the number of simultaneous connections, new connection attempts beyond this limit produce an error until some of the open connections are closed
- Browser will show error like "connection reset" when running this because the server currently doesn't send any data back but there are multiple connections established
- Will sometimes see multiple messages printed for one browser request, the reason might be that the browser is making a request for the page as well as a request for other resources, like the favicon.ico that appears in the browser tab
- Could also be that the browser is trying to connect to the server multiple times because the server isn't responding with any data
- When `stream` goes out of scope and is dropped at the end of the loop, the connection is closed as part of the `drop` implementation, browsers somtimes deal with closed connections by trying because the problem may be temporary
- Browsers also open multiple connections to the server without sending any requests, so that if they do later send requests, they can happen faster, when this happens, the server will see each connection, regardless of whether there are requests over that connection, many versions of Chrome-based browsers do this, for example, can disable that optimization by using private browsing mode or a different browser
- Have successfully gotten a handle to a TCP connection
- Need to stop program by pressing `ctrl`-`c` when done with particular version of the code, then restarting the program by invoking the `cargo run` commad after making changes to the code

### Reading the Request
