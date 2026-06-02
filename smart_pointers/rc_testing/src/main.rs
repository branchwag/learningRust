#[derive(Debug)]
#[allow(dead_code)] // fields exist to demonstrate Rc, not read directly
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a)); //clone that just increases ref count
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        println!("b = {b:?}");
        println!("c = {c:?}");
    }
    /*
    b -> Cons(3, ──┐
                   │
                   ▼
                 [5 -> 10 -> Nil]
                   ▲
                   │
    c -> Cons(4, ──┘
    */
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
