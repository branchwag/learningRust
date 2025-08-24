use std::fmt::Display;

struct Person {
    name: String,
    age: u32,
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} (age {})", self.name, self.age)
    }
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    println!("{}", person);

    let s = person.to_string();
    println!("{}", s);
}
