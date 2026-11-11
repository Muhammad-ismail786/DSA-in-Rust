// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

// Add Two Numbers
struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;

        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;

        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;

            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }

            if sum >= 10 {
                carry = 1;
            } else {
                carry = 0;
            }

            let digit = sum % 10;

            current.next = Some(Box::new(ListNode::new(digit)));

            current = current.next.as_mut().unwrap();
        }

        dummy.next
    }
}

// Create Linked List
fn create_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut current = &mut dummy;

    for value in values {
        current.next = Some(Box::new(ListNode::new(value)));

        current = current.next.as_mut().unwrap();
    }

    dummy.next
}

// Print Linked List
fn print_list(head: &Option<Box<ListNode>>) {
    let mut current = head.as_ref();

    while let Some(node) = current {
        print!("{}", node.val);

        current = node.next.as_ref();

        if current.is_some() {
            print!(" -> ");
        }
    }

    println!();
}

// Main Function
fn main() {
    // First Number: 342
    let l1 = create_list(vec![2, 4, 3]);

    // Second Number: 465
    let l2 = create_list(vec![5, 6, 4]);

    println!("First Number:");
    print_list(&l1);

    println!("Second Number:");
    print_list(&l2);

    // Add Two Numbers
    let result = Solution::add_two_numbers(l1, l2);

    println!("Result:");
    print_list(&result);
}
