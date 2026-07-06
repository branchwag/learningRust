fn main() {
    let v = ['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        //need both iter and enumerate
        //bc iter just gives values
        //enumerate gives index and value. enumerate is an iterator adaptor. It only works on
        //iterators, not directly on collections
        println!("{value} is at index {index}");
    }
}
