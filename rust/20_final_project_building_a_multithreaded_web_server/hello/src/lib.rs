use std::{
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

pub struct Worker {
    id: usize,
    thread: JoinHandle<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub enum PoolCreationError {
    SizeInvalid,
}

impl ThreadPool {
    /// Create a new pool
    ///
    /// The size is the number of maximum threads in the pool.
    ///
    /// # Panic
    ///
    /// The `new` function will pani if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel::<Job>();

        // HOLY SHIT!!!
        //
        // Arc is going to make sure multiple ownerships of receiver(or whatever wraps the receiver,
        // the most important thing here that we are eventually talking about is receiver) are
        // produced.
        //
        // Mutex will make sure each of created Mutex instances are interacting with the receiver
        // once at a time by using the lock stuff.
        //
        // WOW!!! I have never though of this!!! ThIs is aMaziNg.
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::with_capacity(size); // !!!

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
    // pub fn get_threads(&self) -> &PoolThreads {
    //     &self.threads
    // }

    // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
    //     if size > 0 {
    //         return Err(PoolCreationError::SizeInvalid);
    //     }
    //
    //
    //     Ok(ThreadPool {threads})
    // }
    //
    //
    //
    //
    //
    // pub fn execute<F>(&mut self, closure: F)
    // where
    //     F: FnOnce() + Send + 'static,
    // {
    //     println!("EXECUT???");
    //     println!("{}", self.threads_num);
    //     self.threads_num += 1;
    //     let something = thread::spawn(closure);
    //     if something.is_finished() {
    //         self.threads_num -= 1;
    //     };
    //
    //     // loop {
    //     //     if self.threads_num < self.maximum_threads_num {
    //     //         self.print_status("EXECUTING");
    //     //         self.threads_num += 1;
    //     //         thread::spawn(closure);
    //     //         self.threads_num -= 1;
    //     //         break;
    //     //     } else {
    //     //         self.print_status("WAITING");
    //     //
    //     //         thread::sleep(Duration::from_secs(1));
    //     //     }
    //     // }
    // }
    // pub fn get_maximum_threads_num(&self) -> u8 {
    //     self.maximum_threads_num
    // }
    // pub fn get_threads_num(&self) -> u8 {
    //     self.threads_num
    // }
    // fn print_status(&self, status: &str) {
    //     println!(
    //         "{}... {}/{}",
    //         status, self.threads_num, self.maximum_threads_num
    //     );
    // }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();

            println!("Worker {id} finished a job; existing.");
        });

        Worker { id, thread }
    }
}
