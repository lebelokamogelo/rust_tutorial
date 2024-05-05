use std::{sync::mpsc, thread, time::Duration};

pub fn thread_mul() {
    // Sending Multiple Values and Seeing the Receiver Waiting

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    // This time, before we create the first spawned thread, we call clone on
    // the transmitter. This will give us a new transmitter we can pass to the
    // first spawned thread. We pass the original transmitter to a second spawned
    // thread. This gives us two threads, each sending different messages to
    // the one receiver.

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
    // Creating Multiple Producers by Cloning the Transmitter
    //
    // create multiple threads that all send values to the same receiver.
    // We can do so by cloning the transmitter
}
