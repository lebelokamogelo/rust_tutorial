use std::{thread, time::Duration};

// Using Threads to Run Code Simultaneously
// Creating a New Thread with spawn
//
pub fn thread() {
    // spawn to fire up a new thread
    let thread1 = thread::spawn(|| {
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
}
