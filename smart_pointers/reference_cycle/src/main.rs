//memory leak - memory that is never cleaned up
//possible to create references using Rc<T> and RefCell<T> whose items refer to each other in a
//cycle
//ref count never reaches 0 so values will never be dropped
use crate::List::{Cons, Nil};
use std::cell::RefCell; //refcell checks borrowing rules at runtime
use std::rc::Rc;

#[derive(Debug)]
//defines a linked list
enum List {
    Cons(i32, RefCell<Rc<List>>), //Node - stores i32 and mutable pointer to next list node
    Nil,                          //end of list
}

//Rc<List> means multiple parts of theprogram can own the same list node
//RefCell<Rc<List>> means the "next" pointer can be changed even though List is inside an immuteable
//Rc

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    //a -> 5 -> Nil
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a)); //1
    println!("a next item = {:?}", a.tail()); //Some(RefCell { value: Nil })

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    //b -> 10 -> a -> 5 -> Nil

    println!("a rc count after b creation = {}", Rc::strong_count(&a)); //2
    println!("b initial rc count = {}", Rc::strong_count(&b)); //1
    println!("b next item = {:?}", b.tail()); //Some(RefCell { value: Cons(5, RefCell { value:
    //Nil })

    if let Some(link) = a.tail() {
        //mutate a's tail
        *link.borrow_mut() = Rc::clone(&b);
    }
    // a -> 5 -> b -> 10 -> a -> ...

    println!("b rc count after changing a = {}", Rc::strong_count(&b)); //2
    println!("a rc count after changing a = {}", Rc::strong_count(&a)); //2

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    println!("a next item = {:?}", a.tail());
}
