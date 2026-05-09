struct Stack<T> {
    //what fields exist, what types they hold
    items: Vec<T>,
}

impl<T> Stack<T> {
    //what you can do with a Stack
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn size(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    let mut stack = Stack::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Pop: {:?}", stack.pop());
    println!("Peek: {:?}", stack.peek());
    println!("Size: {}", stack.size());
    println!("Is the stack empty? {}", stack.is_empty());
}
