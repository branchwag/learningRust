struct MyBox<T>(T);
//to implement a trait, we need to provide implementations for the trais's required methods
use std::ops::Deref;

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        //returns a reference
        &self.0 //accesses the first value in a tuple struct
    }
}

fn hello(name: &str) {
    //string slice
    println!("Hello, {name}!");
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); //end up with i32

    //deref coersion
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
