fn main() {
    let v = vec![1, 2, 3];
    for x in v.iter() {
        println!("{x}");
    }
    //you're borrowing each element
    //x is &i32 here
    //you can still use v afterward
}
