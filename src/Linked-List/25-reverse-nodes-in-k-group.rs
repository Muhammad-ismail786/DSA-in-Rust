
use std::io::{self, Read};

// Definition for singly linked list
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn reverse_k_group(
        head: Option<Box<ListNode>>,
        k: i32,
    ) -> Option<Box<ListNode>> {

        // Step 1: If k is 1 or less, no reversal is needed
        if k <= 1 {
            return head;
        }

        // Step 2: Check if k nodes are available
        let mut check = head.as_ref();

        for _ in 0..k {
            match check {
                Some(node) => {
                    check = node.next.as_ref();
                }
                None => {
                    return head;
                }
            }
        }

        // Step 3: Reverse k nodes
        let mut prev: Option<Box<ListNode>> = None;
        let mut curr = head;

        for _ in 0..k {
            let mut node = curr.unwrap();

            // Take the next node
            curr = node.next.take();

            // Reverse the link
            node.next = prev;

            // Move prev forward
            prev = Some(node);
        }

        // Step 4: Connect remaining nodes
        let mut reversed_head = prev;

        let mut tail = reversed_head.as_mut().unwrap();

        while tail.next.is_some() {
            tail = tail.next.as_mut().unwrap();
        }

        tail.next = Self::reverse_k_group(curr, k);

        // Step 5: Return new head
        reversed_head
    }
}

// Create linked list from a vector
fn build_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;

    for value in values.into_iter().rev() {
        let mut new_node = Box::new(ListNode::new(value));

        new_node.next = head;
        head = Some(new_node);
    }

    head
}

// Print linked list
fn print_list(head: &Option<Box<ListNode>>) {
    let mut current = head.as_ref();

    while let Some(node) = current {
        print!("{}", node.val);

        if node.next.is_some() {
            print!(" -> ");
        }

        current = node.next.as_ref();
    }

    println!();
}

// Main function
fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let mut values = input.split_whitespace();

    let n: usize = values
        .next()
        .expect("Enter number of nodes")
        .parse()
        .expect("Invalid number of nodes");

    let mut nums = Vec::new();

    for _ in 0..n {
        let value: i32 = values
            .next()
            .expect("Enter node value")
            .parse()
            .expect("Invalid node value");

        nums.push(value);
    }

    let k: i32 = values
        .next()
        .expect("Enter k")
        .parse()
        .expect("Invalid k");

    let head = build_list(nums);

    print!("Original List: ");
    print_list(&head);

    let result = Solution::reverse_k_group(head, k);

    print!("After Reversing: ");
    print_list(&result);
}
