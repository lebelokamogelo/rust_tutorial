use std::{sync::mpsc, thread, time::Duration};

// Using Threads to Run Code Simultaneously
// Creating a New Thread with spawn
//
pub fn thread() {
    let (tx, rx) = mpsc::channel();
    // spawn to fire up a new thread
    let thread1 = thread::spawn(move || {
        // Using Message Passing to Transfer Data Between Threads
        // To accomplish message-sending concurrency, Rust's standard library provides an
        // implementation of channels. A channel is a general programming concept by
        // which data is sent from one thread to another.

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

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
