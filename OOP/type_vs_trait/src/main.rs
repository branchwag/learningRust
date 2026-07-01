trait Greet {
    fn greet(&self);
}

struct User {
    name: String,
}

impl Greet for User {
    fn greet(&self) {
        println!("Hello, {}", self.name);
    }
}

fn main() {
    //greet must be called on a User instance
    let user = User {
        name: String::from("Sam"),
    };

    user.greet();
}
