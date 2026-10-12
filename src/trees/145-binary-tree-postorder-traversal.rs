
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Self::post_order(root, &mut res);
        res
    }

    fn post_order(
        node: Option<Rc<RefCell<TreeNode>>>,
        res: &mut Vec<i32>,
    ) {
        if let Some(node) = node {
            let node = node.borrow();

            Self::post_order(node.left.clone(), res);
            Self::post_order(node.right.clone(), res);
            res.push(node.val);
        }
    }
}
