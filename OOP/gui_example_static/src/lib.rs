//define trait named Draw with method draw
// a trait defines a set of methods or behavior that types can implement
pub trait Draw {
    fn draw(&self);
}

pub struct Screen<T: Draw> {
    //here screen is genric. Draw is resolved at compile time
    pub components: Vec<T>, //every element in the vector must be the same concrete type/
                            //T is a concrete type chosen at compile time
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
