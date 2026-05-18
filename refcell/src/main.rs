use std::cell::RefCell;

fn main() {
    let value = RefCell::new(5);

    //immutable borrow
    let v = value.borrow();
    println!("{}", *v);
}
