//multiple ownership with multiple threads
//Arc - atomically reference counted type, safe to use in concurrent situations
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0)); //i32
    let mut handles = vec![];

    for _ in 0..10 {
        //create ten threads
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            //moves counter into thread
            let mut num = counter.lock().unwrap(); //acquires lock
            *num += 1; //adds one
        }); //num does out of scope at end of closure and lock is released
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    //main thread scquires lock
    println!("Result: {}", *counter.lock().unwrap());
}
