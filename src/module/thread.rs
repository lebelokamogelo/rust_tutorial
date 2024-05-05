use std::{sync::mpsc, thread, time::Duration};

// Using Threads to Run Code Simultaneously
// Creating a New Thread with spawn
//
pub fn thread() {
    let (tx, rx) = mpsc::channel();
    // mpsc stands for multiple producer, single consumer
    // spawn to fire up a new thread
    //
    // using move to move tx into the closure so the spawned thread owns tx
    let thread1 = thread::spawn(move || {
        // Using Message Passing to Transfer Data Between Threads
        // To accomplish message-sending concurrency, Rust's standard library provides an
        // implementation of channels. A channel is a general programming concept by
        // which data is sent from one thread to another.
        // A channel has two halves: a transmitter and a receiver.

        let val = String::from("hi");
        tx.send(val).unwrap();

        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    //for i in 1..5 {
    //  println!("hi number {} from the main thread!", i);
    //thread::sleep(Duration::from_millis(1));
    //}

    // Using move Closures with Threads
    //
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    // Waiting for All Threads to Finish Using join Handles
    thread1.join().unwrap();

    // The receiver has two useful methods: recv and try_recv
    // We’re using recv, short for receive, which will block the main thread’s execution
    // and wait until a value is sent down the channel.
    //
    // try_recv is useful if this thread has other work to do while waiting for messages
    println!("Hello world");

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
