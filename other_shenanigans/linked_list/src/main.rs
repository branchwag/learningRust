use std::fmt::Display;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            next: None,
        }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.data
        })
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = Box::new(Node::new(data));
        
        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            let mut current = self.head.as_mut().unwrap();
            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }
            current.next = Some(new_node);
        }
        self.size += 1;
    }

    pub fn remove_at(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }

        if index == 0 {
            return self.pop_front();
        }

        let mut current = &mut self.head;
        for _ in 0..index - 1 {
            current = &mut current.as_mut().unwrap().next;
        }

        let node_to_remove = current.as_mut().unwrap().next.take();
        if let Some(node) = node_to_remove {
            current.as_mut().unwrap().next = node.next;
            self.size -= 1;
            Some(node.data)
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size {
            return None;
        }

        let mut current = &self.head;
        for _ in 0..index {
            current = &current.as_ref().unwrap().next;
        }

        current.as_ref().map(|node| &node.data)
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.size = 0;
    }
}

pub struct LinkedListIter<'a, T> {
    current: Option<&'a Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> LinkedListIter<'_, T> {
        LinkedListIter {
            current: self.head.as_ref(),
        }
    }
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_ref();
            &node.data
        })
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        let mut current = &self.head;
        let mut first = true;

        while let Some(node) = current {
            if !first {
                write!(f, " -> ")?;
            }
            write!(f, "{}", node.data)?;
            current = &node.next;
            first = false;
        }
        write!(f, "]")
    }
}

fn main() {
    let mut list = LinkedList::new();

    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    println!("After push_front operations: {}", list);

    list.push_back(4);
    list.push_back(5);
    println!("After push_back operations: {}", list);

    if let Some(front) = list.peek_front() {
        println!("Front element: {}", front);
    }

    if let Some(element) = list.get(2) {
        println!("Element at index 2: {}", element);
    }

    print!("Iterating through list: ");
    for item in list.iter() {
        print!("{} ", item);
    }
    println!();

    if let Some(popped) = list.pop_front() {
        println!("Popped element: {}", popped);
    }
    println!("After pop: {}", list);

    if let Some(removed) = list.remove_at(1) {
        println!("Removed element at index 1: {}", removed);
    }
    println!("After removal: {}", list);
    
    println!("List size: {}", list.len());
    println!("Is empty: {}", list.is_empty());
}
