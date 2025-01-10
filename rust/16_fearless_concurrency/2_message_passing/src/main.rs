use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx31, rx1) = mpsc::channel();

    thread::spawn(move || {
        let value: String = String::from("hi!");
        let something = tx31.send(value).unwrap();
        println!("From spawned: {something:#?}");
        // since sending something has given its ownership to the main thread,, which means we can
        // no longer use it!
        // println!("{value:#?}");
    });

    // .recv method BLOCKS the main thread until it got any values.
    // .try_recv method does NOT block the main thread but returns immediately if there is any
    // value. The program can do other jobs while checking it consistently until it finds
    // something. This method is I think more appropriate in the context of threads.
    let somethings = rx1.recv().unwrap();

    println!("From the main: {somethings:#?}");

    // --snip--

    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // we can iterate over the received value. The iterator ends when the transmitter finishes.
    // That is the reason why it pauses for one extra seconds after printing everything.
    for received in rx2 {
        println!("{received}");
    }

    // --snip--

    let (tx3, rx3) = mpsc::channel();

    let tx31 = tx3.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx31.send(val).unwrap();
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
            tx3.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx3 {
        println!("Got: {received}");
    }
}
