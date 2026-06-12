use std::sync::mpsc;
use std::thread;

// mpsc - multiple producer single consumer
// a channel can have multiple sending sends but only one receiving end

fn main() {
    let (tx, rx) = mpsc::channel();
    //mspc::channel() returns a tuple
    //  first element is sending end, the transmitter
    //  second element is the receiving end - the receiver

    //using move to move tx into the closure here
    //the spawned thread needs to own the transmitter to be able to send messages through the
    //channel
    thread::spawn(move || {
        let val = String::from("hi");
        //transmitter has a send method that takes the value we want to send
        //send method returns a Result<T, E> type
        //  if the receiver has already been dropped and there's nowhere to send a value, the
        //  send operation will return an error
        tx.send(val).unwrap();
        //instead of unwrap  - ch 9 has proper error handling
    });

    //receiving the value "hi" in the main thread and printing it
    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
