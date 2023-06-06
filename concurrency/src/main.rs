use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from spawned thread.");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi number {i} from main thread.");
        thread::sleep(Duration::from_millis(1));
    }

    // spawned thread does not print because the main thread died.
    // So lets make sure all threads are finished before exiting the program.

    handle.join().unwrap();
    // If we called join before the main thread for loop it would wait for
    // the spawn thread to finish first before the main thread begins.

    take_ownership_of_data();
    using_channels_to_send_data_between_threads();
    using_channels_and_iterating_the_results_from_it();
    channels_with_multiple_threads();
    using_mutex_to_access_shared_data();
    using_mutex_and_sharing_between_threads();
}

fn take_ownership_of_data() {
    use std::thread;

    let v = vec![1, 2, 3];

    // note the move keyword to move v into the thread.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn using_channels_to_send_data_between_threads() {
    let (transmitter, receiver) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi from spwaned thread.");
        transmitter.send(val).unwrap();
    });

    let received = receiver.recv().unwrap();
    println!("Got {}", received);
}

fn using_channels_and_iterating_the_results_from_it() {
    let (transmitter, receiver) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            transmitter.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in receiver {
        println!("Got: {}", received);
    }
}

fn channels_with_multiple_threads() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
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
}

use std::sync::Mutex;

fn using_mutex_to_access_shared_data() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn using_mutex_and_sharing_between_threads() {
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

    println!("Result: {}", *counter.lock().unwrap());
}
