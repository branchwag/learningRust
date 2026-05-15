fn main() {
    //          012345678910111213
    let text = "apple banana apple";
    let pos = text.rfind("apple");

    println!("{:?}", pos);
}
