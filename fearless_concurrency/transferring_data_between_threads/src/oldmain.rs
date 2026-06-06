use std::sync::mpsc;
//mpsc - multiple producer, single consumer
//channels can have multiple sending ends that produce values
//only one receiving end that consumes those values
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    //tx - transmitter
    //rx - receiver

    thread::spawn(move || {
        //using move to move tx into the closure
        let val = String::from("hi");
        tx.send(val).unwrap();
        //send method returns Result<T, E>
        //upwrap here will panic in case of an error
    });
    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
