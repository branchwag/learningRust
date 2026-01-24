fn main() {
    //making a vec and pushing
    //let mut v: Vec<i32> = Vec::new();
    //v.push(5);
    //v.push(6);
    //v.push(7);
    // println!("{:?}", v);

    //summing
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
    println!("Vec : {:?}", numbers);
}
