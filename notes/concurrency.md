# Fearless Concurrency
- Handling concurrent programming safely and efficiently is one of Rust's major goals
- Concurrent programming, in which different parts of a program execute independently, and parallel programming, in which different parts of a program execute at the same time are important as computers take advantage of multiple processors
- Historically, programming in these contexts is difficiult and error prone
- Onwership and type systems are a powerful set of tools to help manage memory safety and concurrency problems, by leveraging ownership and type checking, many concurrency errors are compile-time errors in Rust, rather than runtime errors
   - Therefore, rather than spending a lot of time trying to reproduce the exact circumstance under which runtime concurrency bugs occur, incorrect code will refuse to compile and will get an error explaining the problem, as a result, can fix code during development rather than after shipping to production
- Fearless concurrency allows the writing of code that is free of subtle bugs and is easy to refactor without introducing new bugs
- For this section, concurrent will encapsulte both concurrent and/or parallel
- How to create threads to run multiple pieces of code at the same time
- Messagem-passing concurrency, where channels send messages between threads
- Shared-state concurrency, where multiple threads have access to some piece of data
- The `Sync` and `Send` traits, which extend Rust's concurrency guarantees to user-defined types as well as types provided by the standard library

## Using Threads to Run Code Simultaneously
- In most current operating systems, an executed program's code is run in a process, and the operating system will manage multiple processes at once, within a program can also have indepedent parts that run simultaneously
   - The features that run these independent parts are called threads
   - A web server could have multiple threads so that it can respond to more than one request at the same time
- Spltting the computation in a program into multiple threads to run multiple tasks at the same time can imrpove performance but also adds complexity
- Since threads can run simultaneously, there's no inherent guarantee about the order in which parts of the on different threads will run, this can lead to probles:
   - Race conditions: in which threads are accessing data or resources in an inconsistent order
   - Deadlocks, in which two threads are waiting for each other, preventing both threads from continuing
   - Bugs that happen only in certain situations and are hard to reproduce and fix reliably
- Rust attempts to mitigate the negative affects of using threads, but programming in a multithreaded context still takes careful thoguht and requres a code structure that is different from programs running in a single thread
- Programming languages implement threads in a few different ways, and many operating systems provide an API the language can call for creating new threads
- The Rust standard library uses a 1:! model of thread implementation, whereby a progarm uses one operating system thread per language thread, there are crates that implement other models of threads that make different tradeoffs to the 1:! model (Rust's async system)

### Creating a New Thread with `spawn`
- To create a new thread, can call `thread::spawn` function and pass ot to a closure, containing the code to run in the new thread
- Example: ```
    thread::spawn(|| {
        for i in 1..20 {
            println!("{i} from spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });```
- This prints some text from a main thread and other text from a new thread
- When the main thread of a Rust program completes, all spawned threads are shut down whether or not they have finished running
- The calls to `thread::sleep` force a thread to stop its execution for a short duration allowing a different thread to run
- Threads will probably take turns but that isn't guaranteed, it depends on how the operating systemm scheduldes the threads, in this run the main thread is printed first, even though the operating system spawned thread appears first in the code and even though the spawned thread was told to print until `i` is `9`, it only got to `5` before the main thread got shut down
- To only see output from the main thread or not see any overlap, by increasing the number in the ranges to create more opportunities for the operating system to switch between the threads

### Waiting for All Threads to Finish Using `join` Handles
- The previous code stops the spawned thread prematurely most of the time due to the main thread ending, but because there is no guarantee on the order in which threads run, also can't guarantee that the spawned thread will get to run at all
- Can fix the problem of the spawned thread not running or ending prematurely by saving the return value of `thread::spawn` in a variable, the return type of `thread_spawn` is `JoinHandle<T>`, a `JoinHandle<T>` is an owned value that, when called the `join` method on it will wait for its thread to finish, returns a `Result<T>`
- Example: ```
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{i} from spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("{i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}```
- Calling `join` on the handle blocks the thread currently running  until the thread represented by the handle terminates, blocking a thread means that thread is prevented from performing work or exiting
- The two threads continue alternating, but the main thread waits because of the call to `handle.join()` and does not end until the spawned thread is finished
- Moving the `handle.join()` before the `for` loop in `main` would cause the thread to complete execution before the `for` loop
- The main thread will wait for the spawned thread to finish and then run its `for` loop so that the output won't be interleaved anymore
- Small details such as where `join` is called can affect whether or not threads run at the same time

### Using `move` Closures with Threads
- The `move` keyword with closures if often passed to `thread::spawn` because the closure will take ownership of the values it uses from the environment, thus transferring ownership of these values from one thread to another
- To use data from the main thread in the spawned thread, the spawned thread's closure must capture the value it needs
- Example: ```
 let v = vec![1, 2, 3];

 let handle = thread::spawn(|| {
     println!("vec {v:?}");
 });

 handle.join().unwrap();```
- The closure uses `v` so it will capture `v` and make it part of the closure's environment, since `thread::spawn` runs a closure in a new thread, should be able to access `v` inside that new thread but receive compile-time error that the clpsure may outlive the current function but it borrows something
- Rust infers how to capture `v` and becaue `prinln!` only needs a reference to `v`, the clousre tries to borrow `v`, there's a problem: Rust can't tell how long the spawned thread will run so it doesn't know whether the reference to `v` will always be valid
- By adding the `move` keyword before the closure to take ownership of the values it's using rather than allowing Rust to infer that it should borrow the values
- Example: ```
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("vec {v:?}");
    });

    handle.join().unwrap();```
- The `move` keyword overrides Rust's conservative default of borrowing, it doesn't allow violation of the ownership rules

## Using Mesage Passing to Transfer Data Between Threads
- One approach to ensuring safe concurrency is message passing, where threads or actors communicate by sending each other messages containing data, the idea is not to communicate by sharing memory but instead sharing memory by communicating
- To accomplish message-sending concurrency, Rust's standard library provides an implementation of channels, a channel is a concept by which data is sent from one thread to another
- Can image a channel as a bidrectional flow of water, such as a stream or river, placing something like a rubber duck into a river, it will travel downstream to the end of the waterway
- A channel has two halves: a transmitter and receiver
   - The transmitter half is the upstream location where the item is placed into the river and the receiver half is where the item ends up downstream
   - One part of code calls methods on the transmitter with the data to send and another part checks the receiving end for arriving messages
   - A channel is said to be closed if either the transmitter or receiver half is dropped
- Program that has one thread to generate values and send them down a channel and another thread that will receive these values and print them out, values will be sent between threads using a channel
- Channels can be used for any threads that need to communicate with each other, such as a chat system or a system where many threads perform parts of a calculation and send the parts to one thread that aggregates the results
- Example: ```
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}```
   - This won't compile because Rust can't tell hat type of values to send over the channel
   - This creates a new channel using the `mpsc::channel` function returns a tuple, the first element of which is the sending end, the transmitter, and the second element of which is the receiving end, the receiver, the abbreviations `tx` and `rx` are traditionally used in many fields for transmitter and receiver, resepctively, so variables are named as such to indicate each end
   - A `let` statement with a pattern that destructures the tuples  is used, a let statement is a convenient way to extract the pieces of the tuple returned by `mpsc::channel`
   - Example: ```
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}```
- This moves the transmitting end into a spawned thread and has it send one string so the spawned thread is communicating with the main thread, this is like sending data from one thread to another
- Again, `thread::spawn` is used to create a new thread, and then, using `move` to move `tx` into the closure so the spawned thread owns `tx`, the spawned thread needs to own the transmitter to be able to send messages through the channel
- The transmitter has a `send` method that takes the value to send, the `send` method returns a `Result<T, E>` type, so if the receiver has already been dropped and there's nowhere to send a value, the send operation will return an error
- In this example, `unwrap` is called to panic in case of an error, but in a real application, it would be handled properly
- Example: ```
fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("got {received}");
}```
- This gets a value from the receiver in the main thread, this is similar to receiving a chat message or retrieving an item from the water
- The receiver has two useful methods: `recv` and `try_recv`, when using `recv`, short for receive, which will block the main thread's execution and wait until a value is sent down the channel, once a value is sent `recv` will return it in a `Result<T, E>`, when the transmitter closes, `recv` will return an error to signal that no more values will be coming
- The `try_recv` method doesn't block, but will instead return a `Result<T, E` immediately, an `Ok` value holding a message if one is available and an `Err` value if there aren't any messages this time
   - Using `try_recv` is useful if this thread has other work to do while waiting for messages, could write a loop that calls `try_recv` every so often, handles a message if one is available, and otherwise does other work for a little while until checking again
   - Have used `recv` in this example for simplicity, don't have any other work for the main thread to do other than wait for messages, so blocking the main thread is appropriate

### Channele and Ownership Transference
- The ownership rules play a vital role in message sneding because they help write safe, concurrent code, preventing errors in concurrent programming is the advantage of thinking about ownership throughout Rust programs
- Trying to use a value after sending it down a channel won't work because the value is moved into the channel when calling `tx.send`
   - Allowing this would be a bad idea: once the value has been sent to another thread, that thread could modify or drop it befor etrying to use the value again, the other threads modifications could potentially cause errors or unexpected results due to inconsistent, this concurrency mistake would cause a compile time error, the `send` function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it, this prevents accidentally using the value again after sending it, the ownership system checks that everything is fine

### Sending Multiple Values and Seeing the Receiver Waiting
- Example: ```
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    //     println!("{val}");
    // });
    //
    // let received = rx.recv().unwrap();
    // println!("got {received}");

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("abc"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("got {received}");
    }
}```
- Code sends multiple messages and pause for a second between each message
- The spawned thread has a vector of strings that is sent to the main thread, this is iterated over, sending each individually and pauses between each by calling the `thread::sleep` function with a `Duration` value of one section
- In the main thread, instead of calling the `recv` function explicitly, `rx` is treated as an iterator, for each value received, it's printed, when the channel closes, iteration will end
- Since there is no code that pauses or delays in the `for` loop in the main thread, the main thread is waiting ot receive values from the spawned thread

### Creating Multiple Producers by Cloning the Transmitter
- Example: ```
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    //     println!("{val}");
    // });
    //
    // let received = rx.recv().unwrap();
    // println!("got {received}");

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("abc"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("abc"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("got {received}");
    }
}```
- This creates multiple threads that all send values to the same receiver, can do so by cloning the transmitter
- Before creating the first spawned thread, `clone` is called on the transmitter, this gives a new transmitter that can pass to the first spawned thread, can pass the original transmitter to a second spawned thread, this gives two threads, each sending different messages to one receiver

## Shared-State Concurrency
- Message passing is a fine way to handle concurrency, but not the only way, another method would be for multiple threads to access the same shared data
- What would communicating by sharing memory look like and why would message-passing enthusiasts caution not to use memory sharing?
- In a way, channels in any programming language are similar to single ownership, since once a value is trasnferred down a channel, it should no longer be accessible and that value should not be useable
- Shared memeory concurrency is like multiple ownership, multiple threads can access the same memory location at the same time
- Multiple ownership can add complexity because these different owners need managing, Rust's type system and ownership rules greatly assist in getting this management correct
- An example is mutexes, one of the more common concurrency primitives for shared memory

### Using Mutexes to Allow Access to Data from One Thread at a Time
- Mutex is an abbrevation for mutual exclusion, to access the data in a mutex, a thread must first signal that it wants to access by asking to acquire the mutex's lock
   - The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data, therefore the mutex is described as guarding the data is holds via the locking system
- Mutexes have a reputation for being difficult to use because the need to acknowledge two rules:
1. The lock must be acquired before using the data
2. When done with the data the mutex guards, the data must be unlocked so other threads can acquire the lock

#### The API of `Mutex<T>`
- Example: ```
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
}```
- This is an example of using a mutex in a single threaded context
- `Mutex<T>` is created using the associated function `new`, to access the data inside the mutex, can use hte `lock` method to acqure the lock, this call will block the current thread so it can't do any work until its the main thread's turn to have the lock
- The call to `lock` would fail if another thread holding the lock panicked, in that case no threads would be able to get the lock, so `unwrap` and panic if in that situation
- After acquiring the lock, can treat the return value, `num` in this case, as a mutable reference to the data inside, the type system ensures that a lock is acquired before using the value in `m`, the type of `m` is `Mutex<i32>`, not `i32`, must call `lock` to be able to use the `i32` value, type system won't allow access to the inner `i32` otherwise
- `Mutex<T>` is a smart pointer, the call to `lock` returns a smart pointer called `MutexGuard`, wrapped in a `LockResult` that is handled with the call to `unwrap`, the `MutexGuard` smart pointer implements `Deref` to point at inner data, the smart pointer also has a `Drop` implementation that releases the lock automatically when a `MutexGuard` goes out of scope which happens at the end of the inner scope, as a result there is no risk forgetting to release the lock and blocking the mutex from being used by other threads, since the lock release happens automatically

#### Sharing a `Mutex<T>` Between Multiple Threads
- Sharing a value between multiple threads using `Mutex<T>`, will spin up 10 threads and have them each increment a counter value by 1, so counter goes from 0 to 10
- Example: ```
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

}```
- This creates a `counter` variable to hold an `i32` inside a `Mutex<T>`, then creates 10 threads by iterating over a range of numbers, `thread::spawn` is used and all threads have the same closure, one that moves the counter into the thread, acquires the lock on the `Mutex<T>` by calling the `lock` method, and then adds `1` to the value in the mutex, when a thread finishes its closure, `num` will go out of scope and release the lock so another thread can acquire the lock and print the result
- This code won't compile because the `counter` value was moved in the previous iteration of the loop, Rust can't move the ownership of lock `counter` into multiple threads

#### Multiple Onwership with Multiple Threads
- Can use the smart pointer `Rc<T>` which gives a value to multiple owners and creates a reference counted value
- Can wrap the `Mutex<T>` in `Rc<T>` and clone the `Rc<T>` before moving ownership to the thread
- Example: ```
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap());

}```
- This results in a compiler error, noting that `Rc<Mutex<i32>>` cannot be sent between threads safely since the trait `Send` is not implemented for `Rc<Mutx<i32>>`, `Send` is one of the traits that ensures the types used with threads are meant for use in concurrent situations
- `Rc<T>` is not safe to share across threads, when `Rc<T>` manages the reference count, it adds the count for each call to `clone` and subtracts from the count when each clone is dropped, but it doesn't use any concurrency primtives to ensure that changes to the count can't be interrupted by another thread, this could lead to wrong counts, bugs that turn into memory leaks, or a value being dropped prematurely, need a type like `Rc<T>` but one that makes changes to the reference count in a thread-safe way

#### Atomic Reference Counting with `Arc<T>`
- Fortuntely, `Arc<T>` is a type like `Rc<T>` that is safe to use in concurrent situatiosn
- The *a* stands for atomic, meaning it's an atomically reference-counted type, Atomics are an additional kind of concurrency primitive, atomics work like primitve types but are safe to share across threads
- This safety comes with a performance penalty that should only be paid when necessary
- Example: ```
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap());

}```
- `Arc<T>` and `Rc<T>` have the same API
- Can also use this program's structure to do more complicated operations than just incrementing a counter, using this strategy, can divide a calculation into independent parts, split those parts across threads, and then use a `Mutex<T>` to have each thread update the final result with its part
- Note that if doing simple numerical operations, there are types ismpler than `Mutex<T>` types provided by the `std::sync::atomic` module of the standard library, these types provide safe, concurrent, atomic access to primitive types

### Similarities Between `RefCell<T>/Rc<T>` and `Mutex<T>/Arc<T>`
- `counter` is immutable but could get a mutable reference to the value inside it, this means `Mutex<T>` provides interior mutability, as the `Cell` family does
- In the same way, `RefCell<T>` was used to mutate contents inside an `Rc<T>`, can use `Mutex<T>` to mutate contents inside an `Arc<T>`
- Another detail is that Rust can't protect from all kinds of logic errors when using `Mutex<T>`, using `Rc<T>` comes at the risk of creating reference cycles, where two `Rc<T>` values refer to each other, causing memory leaks, similarly, `Mutex<T>` comes with the risk of creating deadlocks, these occur when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait in for each other forever

## Extensible Concurrency with the `Send` and `Sync` Traits
- Can create custom concurrency features or use those written by others
- Among the key concurrency concepts embedded in the langauge, rather than the standard library, are the `std::marker` traits `Send` and `Sync`

### Allowing Transference of Onwership Between Threads with `Send`
- The `Send` marker trait indicates that ownership of values of the type implementing `Send` can be transferred between threads, almost every Rust type is `Send` but there are some exceptions including `Rc<T>`, this cannot implement `Send` because it an `Rc<T>` was cloned and tried to transfer ownership of the clone to another thread, both threads might update the reference count at the same time, for this reason, `Rc<T>` is implemented for use in single-threaded situations where there is no need to pay the thread-safe performance penalty
- Rust's type system and trait bounds ensure that an `Rc<T>` value can never accidentally be sent across threads unsafely, when trying to do this, receive an error that the trait `Send` is not implemented for `Rc<T>`, when switching to `Arc<T>`, this does compile
- Any type composed of `Send` types is automatically marked as `Send` as well, almost all primitive types are `Send`, aside from raw pointers

### Allowing Access from Multiple Threads with `Sync`
- The `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be referred to from multiple threads, in other words, any type `T` implements `Sync` if `&T` (an immutable reference to `T`) implements `Send`, meaning the reference can be sent safely to another thread, similar to `Send`, primitive types all implement `Send`, and types composed entirely of types that implement `Sync` also implement `Sync`
- The smart pointer `Rc<T>` also doesn't implement `Sync` for the same reasons it doesn't implement `Send`, the `RefCell<T>` type and the family of related `Cell<T>` types don't implement `Sync`, the implementation of borrow checking that `RefCell<T>` does at runtime is not thread-safe
- The smart pointer `Mutex<T>` implements `Sync` and can be used to share access with multiple threads 

### Implementing `Send` and `Sync` Manually is Unsafe
- Since types composed entirely out of other types that implement the `Send` and `Sync` traits also automatically implement `Send` and `Sync`, don't have to implement these traits manually, as marker traits, they don't even have any methods to implement, just useful for enforcing invariants related to concurrency
- Manually implementing these traits involves implementing unsafe Rust code, building new types not made up of `Send` and `Sync` parts requires careful thought to uphold the safety guarantees

### Summary
- Since very little of how Rust handles concurrency is part of the language, many concurrency solutions are implemented as crates, these evolve more quickly than the standard library 
- The Rust standard library provides channels for message passing and smart pointer types such as `Mutex<T>` and `Arc<T>` that are safe to use in concurrent contexts, the the type system and the borrow checker ensure that the code using these solutions won't end up with data races or invalid references, once code compiles, can rest assured that it will run on multiple threads without kinds of difficult to track down bugs
