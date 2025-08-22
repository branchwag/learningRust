fn main() {
    let s1 = String::from("hello");
    //refs allows referral without taking ownership
    let len = calculate_length(&s1); //creating ref is borrowing
    println!("The length of '{s1}' is {len}.");
}

//takes in reference (no ownership here!)
fn calculate_length(s: &String) -> usize {
    s.len()
}

//deref would be *
