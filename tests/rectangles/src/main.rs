#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 100,
        height: 60,
    };

    println!("rect1 is {rect1:?}");
    println!("rect2 is {rect2:?}");

    println!("The area of rectangle 1 is {} square pixels.", rect1.area());

    println!("The area of rectangle 2 is {} square pixels.", rect2.area());

    println!(
        "Will the second rectangle hold the first rectangle? {}",
        rect2.can_hold(&rect1)
    );
}
