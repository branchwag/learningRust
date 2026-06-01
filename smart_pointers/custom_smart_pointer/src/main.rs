struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");
} //dont need to call drop method explicitly
//
//variables are dropped in reverse order of their creation
//
//use std::mem::drop to force drop a value before the end of its scope
