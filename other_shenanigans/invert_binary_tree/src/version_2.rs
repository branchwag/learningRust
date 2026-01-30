#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// Inverts the tree in-place
fn invert_tree(root: &mut Option<Box<TreeNode>>) {
    if let Some(node) = root {
        // Swap left and right recursively
        invert_tree(&mut node.left);
        invert_tree(&mut node.right);
        std::mem::swap(&mut node.left, &mut node.right);
    }
}

/// Returns height of the tree
fn get_height(node: &Option<Box<TreeNode>>) -> usize {
    match node {
        None => 0,
        Some(n) => 1 + get_height(&n.left).max(get_height(&n.right)),
    }
}

/// Prints the tree (simple ASCII)
fn print_tree(root: &Option<Box<TreeNode>>) {
    let height = get_height(root);
    let width = 2_usize.pow(height as u32) - 1;

    let mut levels: Vec<Vec<String>> = vec![vec![" ".to_string(); width]; height * 2];
    fill_tree(root, &mut levels, 0, 0, width - 1);

    for level in levels {
        println!("{}", level.join(""));
    }
}

fn fill_tree(
    node: &Option<Box<TreeNode>>,
    levels: &mut Vec<Vec<String>>,
    level: usize,
    left: usize,
    right: usize,
) {
    if let Some(n) = node {
        let mid = (left + right) / 2;
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
    // Build the tree
    let mut root = Some(Box::new(TreeNode::new(1)));
    root.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));
    root.as_mut().unwrap().right = Some(Box::new(TreeNode::new(3)));
    root.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(4)));
    root.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(5)));

    println!("Original tree:");
    print_tree(&root);

    invert_tree(&mut root);

    println!("\nInverted tree:");
    print_tree(&root);
}
