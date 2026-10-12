
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        Self::pre_order(root, &mut result);
        result
    }

    fn pre_order(
        node: Option<Rc<RefCell<TreeNode>>>,
        result: &mut Vec<i32>,
    ) {
        if let Some(node) = node {
            let node = node.borrow();

            result.push(node.val);
            Self::pre_order(node.left.clone(), result);
            Self::pre_order(node.right.clone(), result);
        }
    }
}
