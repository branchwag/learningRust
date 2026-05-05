fn main() {
    /*
    for _ in 0..10 {
        println!("Are we there yet?");
    }
    */

    let mut i = 0;
    loop {
        if i == 10 {
            break;
        }
        println!("Are we there yet?");
        i += 1;
    }
}
