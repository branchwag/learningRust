use std::thread::JoinHandle;
use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;
// a job is just a closure
// different closures have different concrete types (compiler-generated types)
// use trait object dyn FnOnce() to erase concrete types
// now the queue can hold any closure that can be called once
// trait object dyn FnOnce() is unsized (Rust doesn't know how big it is at compile time)
// so put it in a Boz so it has a known size (a pointer)
// Send - safe to transfer between threads

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        thread::spawn(f)
    }

    pub fn execute<F>(&self, f: F)
    //sends a job into the channel
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    //Arc - shared ownership
    //Mutex - safe shared access
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            //need closure to loop forever,
            //asking the receiving end of the channel for a job
            //and running the job when it gets one
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                //call lock to acquire mutex
                //might fail if mutex is in poisoned state (thread panicked while holding
                //lock)
                //if we get lock, call rec to receive aa Job from the channel
                println!("Worker {id} got a job; executing.");
                job();
            }
        });

        Worker { id, thread }
    }
}
