
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<ListNode>>>;

#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Link,
}

impl ListNode {
    fn new(val: i32) -> Rc<RefCell<ListNode>> {
        Rc::new(RefCell::new(ListNode {
            val,
            next: None,
        }))
    }
}

struct Solution;

impl Solution {
    fn is_palindrome(head: Link) -> bool {
        let mut slow = head.clone();
        let mut fast = head.clone();

        // Find the middle of linked list
        while let Some(fast_node) = fast.clone() {
            let next = fast_node.borrow().next.clone();

            if next.is_none() {
                break;
            }

            slow = slow.and_then(|node| node.borrow().next.clone());

            fast = next.and_then(|node| node.borrow().next.clone());
        }

        // Reverse the second half
        let mut prev: Link = None;
        let mut curr = slow;

        while let Some(node) = curr {
            let next = node.borrow().next.clone();

            node.borrow_mut().next = prev.clone();

            prev = Some(node);
            curr = next;
        }

        // Compare both halves
        let mut first = head;
        let mut second = prev;

        while let Some(second_node) = second {
            let first_node = match first {
                Some(node) => node,
                None => return false,
            };

            if first_node.borrow().val != second_node.borrow().val {
                return false;
            }

            first = first_node.borrow().next.clone();
            second = second_node.borrow().next.clone();
        }

        true
    }
}

fn main() {
    // Hardcoded input: 1 -> 2 -> 2 -> 1

    let head = ListNode::new(1);
    let node2 = ListNode::new(2);
    let node3 = ListNode::new(2);
    let node4 = ListNode::new(1);

    head.borrow_mut().next = Some(node2.clone());
    node2.borrow_mut().next = Some(node3.clone());
    node3.borrow_mut().next = Some(node4.clone());

    let result = Solution::is_palindrome(Some(head));

    println!("Is Palindrome: {}", result);
}
