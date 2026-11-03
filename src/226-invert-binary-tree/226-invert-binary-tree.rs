use std::cell::RefCell;
use std::rc::Rc;

// Step 1: TreeNode structure
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

// Step 2: TreeNode constructor
impl TreeNode {
    fn new(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        }))
    }
}

// Step 3: Invert Binary Tree function
fn invert_tree(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {

    if root.is_none() {
        return None;
    }

    let (left, right) = {
        let node_rc = root.as_ref().unwrap();
        let mut node = node_rc.borrow_mut();

        let TreeNode { left, right, .. } = &mut *node;

        std::mem::swap(left, right);

        (left.clone(), right.clone())
    };

    invert_tree(left);
    invert_tree(right);

    root
}

// Step 4: Tree print function
fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) {

    if let Some(node_rc) = root {
        let node = node_rc.borrow();

        print!("{} ", node.val);

        let left = node.left.clone();
        let right = node.right.clone();

        drop(node);

        print_tree(left);
        print_tree(right);
    }
}

// Step 5: Main function
fn main() {

    let root = TreeNode::new(4);

    let node2 = TreeNode::new(2);
    let node7 = TreeNode::new(7);

    let node1 = TreeNode::new(1);
    let node3 = TreeNode::new(3);
    let node6 = TreeNode::new(6);
    let node9 = TreeNode::new(9);

    {
        let mut root_node = root.borrow_mut();

        root_node.left = Some(node2.clone());
        root_node.right = Some(node7.clone());
    }

    {
        let mut node = node2.borrow_mut();

        node.left = Some(node1);
        node.right = Some(node3);
    }

    {
        let mut node = node7.borrow_mut();

        node.left = Some(node6);
        node.right = Some(node9);
    }

    println!("Original Tree:");
    print_tree(Some(root.clone()));

    let root = invert_tree(Some(root)).unwrap();

    println!("\nInverted Tree:");
    print_tree(Some(root));

    println!();
}
