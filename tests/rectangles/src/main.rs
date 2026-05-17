use rectangles::Rectangle;

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
