use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            val: value,
            left: None,
            right: None,
        }
    }
}

fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root.clone() {
        let mut n = node.borrow_mut();
        let left = n.left.take();
        let right = n.right.take();
        n.left = invert_tree(right);
        n.right = invert_tree(left);
    }
    root
}

fn get_height(node: &Option<Rc<RefCell<TreeNode>>>) -> usize {
    match node {
        None => 0,
        Some(n) => {
            let n = n.borrow();
            1 + get_height(&n.left).max(get_height(&n.right))
        }
    }
}

fn print_tree(root: &Option<Rc<RefCell<TreeNode>>>) {
    let height = get_height(root);
    let width = 2_usize.pow(height as u32) - 1;

    let mut levels: Vec<Vec<String>> = vec![vec![" ".to_string(); width]; height * 2];
    fill_tree(root, &mut levels, 0, 0, width - 1);

    for level in levels {
        println!("{}", level.join(""));
    }
}

fn fill_tree(
    node: &Option<Rc<RefCell<TreeNode>>>,
    levels: &mut Vec<Vec<String>>,
    level: usize,
    left: usize,
    right: usize,
) {
    if let Some(n) = node {
        let mid = (left + right) / 2;
        let n = n.borrow();

        levels[level * 2][mid] = n.val.to_string();

        if n.left.is_some() {
            let left_mid = (left + mid - 1) / 2;
            levels[level * 2 + 1][left_mid..mid].fill("/".to_string());
            fill_tree(&n.left, levels, level + 1, left, mid - 1);
        }

        if n.right.is_some() {
            let right_mid = (mid + 1 + right) / 2;
            levels[level * 2 + 1][(mid + 1)..=right_mid].fill("\\".to_string());
            fill_tree(&n.right, levels, level + 1, mid + 1, right);
        }
    }
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let left = Rc::new(RefCell::new(TreeNode::new(2)));
    let right = Rc::new(RefCell::new(TreeNode::new(3)));
    let left_left = Rc::new(RefCell::new(TreeNode::new(4)));
    let left_right = Rc::new(RefCell::new(TreeNode::new(5)));

    left.borrow_mut().left = Some(left_left);
    left.borrow_mut().right = Some(left_right);
    root.borrow_mut().left = Some(left);
    root.borrow_mut().right = Some(right);

    println!("Original tree:");
    print_tree(&Some(root.clone()));

    invert_tree(Some(root.clone()));

    println!("\nInverted tree:");
    print_tree(&Some(root.clone()));
}
