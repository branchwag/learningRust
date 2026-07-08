fn main() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"), //includes five
        _ => println!("something else"),
    }
    //works for chars too:
    // let x = 'c';
    // match x {
    // 'a'..='j' => println!("early ASCII letter"),
    // 'k'..='z' => println!("late ASCII letter"),
    // _ => println!("something else"),
    // }
}
