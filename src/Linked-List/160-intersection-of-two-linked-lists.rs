
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn get_intersection_node(
        head_a: Option<Rc<RefCell<ListNode>>>,
        head_b: Option<Rc<RefCell<ListNode>>>,
    ) -> Option<Rc<RefCell<ListNode>>> {
        let mut p_a = head_a.clone();
        let mut p_b = head_b.clone();

        while !Self::same_node(&p_a, &p_b) {
            p_a = match p_a {
                Some(node) => node.borrow().next.clone(),
                None => head_b.clone(),
            };

            p_b = match p_b {
                Some(node) => node.borrow().next.clone(),
                None => head_a.clone(),
            };
        }

        p_a
    }

    fn same_node(
        a: &Option<Rc<RefCell<ListNode>>>,
        b: &Option<Rc<RefCell<ListNode>>>,
    ) -> bool {
        match (a, b) {
            (Some(x), Some(y)) => Rc::ptr_eq(x, y),
            (None, None) => true,
            _ => false,
        }
    }
}
