use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

fn main() {
    let m: Mutex<u8> = Mutex::new(5);

    println!("{}", m.lock().unwrap());

    {
        // it BLOCKs the main thread until it gets the lock.... but why?
        // If anothe thread that holds the lock for m panics, it returns Err.
        let mut num = m.lock().unwrap();

        // we do not have to say like m.unlock or that kind of stuff because Mutex<T> is a smart
        // pointer that implements the Drop traits.
        *num = 6;
    }

    println!("{}", m.lock().unwrap());

    let counter: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for _ in 0..10 {
        let cloned_counter: Arc<Mutex<u8>> = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = cloned_counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for current_handle in handles {
        current_handle.join().unwrap();
    }

    println!("Results: {}", counter.lock().unwrap());
}
