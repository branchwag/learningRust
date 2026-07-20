use std::thread::{JoinHandle, Thread};
use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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

        ThreadPool {
            workers,
            sender: Some(sender),
        }
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
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

//implementing the Drop trait to call join on each of the threads in the pool so that they can
//finish the requests they're working on before closing
impl Drop for ThreadPool {
    fn drop(&mut self) {
        //dropping sender closes the channel
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);
            worker.thread.join().unwrap();
        }
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
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    //expicitly breaking out of the loop when recv returns an error
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}
