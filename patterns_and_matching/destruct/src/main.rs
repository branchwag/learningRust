struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    // pattern, expression
    // how to unpack / what to unpack
    let Point { x, y } = p;
    //aserts don't print
    assert_eq!(0, x);
    assert_eq!(7, y);
}
