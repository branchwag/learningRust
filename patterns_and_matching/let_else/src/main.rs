fn main() {
    let some_option_value = Some(42);
    //or
    //let some_option_value: Option<i32> = None;

    let Some(x) = some_option_value else {
        return;
    };
    println!("{x}");
}
