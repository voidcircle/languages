use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

fn main() {
    let handle1: JoinHandle<()> = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // if we call the join method, then the main thread kind of freezes until the handle is
    // completed.
    //
    // If we have this at the end of the main function, then it is going to run the spawned and
    // main thread simultaniously and waits for the spawned thread to finish even though the main
    // thread is finished.
    handle1.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    let mut v: Vec<u8> = vec![1, 2, 3];

    // the spawned thread tries to make a reference to v to copy it to the new thread's
    // environment, but Rust does not know whether or not the reference to v will only be valid
    // util the spawned valid is finished. For example, v might be dropped let's say because the main
    // thread has been finished, but the spawned thread might be still running because the code
    // includes code like `handle2.join();` or something.
    let handle2: JoinHandle<()> = thread::spawn(move || {
        println!("{v:#?}");
        v.push(2);
        println!("{v:#?}");
    });

    // However, as soon as the move keyword is added at the beginning of the closure, we can no
    // longer use the variable since as the keyword says, the variable v has been moved into the
    // closure. The ownership has been moved.
    // println!("{v:#?}");

    handle2.join().unwrap();
}
