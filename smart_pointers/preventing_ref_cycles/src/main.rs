use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

//a tree with nodes that know about their child nodes
#[derive(Debug)]
//node owns children
//share that ownership with variables so we can access each Node in the tree directly
struct Node {
    value: i32,
    // a parent node should own its children. If a parent node is dropped, its child nodes should
    // be dropped as well
    parent: RefCell<Weak<Node>>, //avoid ref cycle with weak
    children: RefCell<Vec<Rc<Node>>>, //vec of nodes. RefCell bc we want to modify which nodes
                                 //are children of another node
}

fn main() {
    let leaf = Rc::new(Node {
        //two owners - leaf and branch
        value: 3,
        parent: RefCell::new(Weak::new()), //no parent
        children: RefCell::new(vec![]),    //no children
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), //1
        Rc::weak_count(&leaf),   //0
    );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()), //no parent node
            children: RefCell::new(vec![Rc::clone(&leaf)]), //leaf as one of its children
        });

        //modding leaf to give it a Weak<Node> reference to its parent
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch), //1
            Rc::weak_count(&branch),   //1
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf), //2 bc branch now has a clone of the Rc<Node> of leaf stored
            //in branch.children
            Rc::weak_count(&leaf), //0
        );
    } //inner scope ends

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    //None
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), //1
        Rc::weak_count(&leaf,)   //0
    );
}
