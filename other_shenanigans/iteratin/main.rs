fn main() {
    let v = vec![10, 20, 30];
    let it = v.iter().enumerate();
    //println!("{:?}", it);

    for (index, value) in it {
        println!("index = {}, value = {}", index, value);
    }
}
