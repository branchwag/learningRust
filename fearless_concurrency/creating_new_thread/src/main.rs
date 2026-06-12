use std::thread;
use std::time::Duration;

fn main() {
    // fix problem of spawned thread ending prematurely by saving the return value in a variable
    // the return type of thread::spawn is JoinHandle<T>
    // owned value
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap(); //moving it here allows for the spawned thread to finish and then run
    //its for loop so the output won't be interleaved anymore

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    //calling join here blocks the thread currently running until the thread represented by the handle
    //terminates
}
