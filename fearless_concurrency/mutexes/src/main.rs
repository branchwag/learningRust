use std::sync::Mutex;
use std::thread;

//Mutex is a type

fn main() {
    let m = Mutex::new(5); //create a new one with 'new'
    {
        let mut num = m.lock().unwrap(); //access data inside mutex with lock method to
        //acquire lock
        //blocks current thread so that it can't do any work until it's our turn to have
        //the lock
        //call to lock returns type called MutexGuard
        //  implements Deref to point at our inner data
        //  Drop implementation that releases a lock automatically when a MutexGuard goes
        //  out of scope
        //  wrapped in a LockResult
        *num = 6;
    }
    println!("m = {m:?}");
}
