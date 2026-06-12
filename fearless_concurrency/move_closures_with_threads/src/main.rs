use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    //the closure uses v
    //but RUST can't tell how long the spawned thread will run, so it doesn't know whether the
    //reference to v will always be valid
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
