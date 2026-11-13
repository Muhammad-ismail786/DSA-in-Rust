use std::cmp::Reverse;
use std::collections::BinaryHeap;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn merge_k_lists(
        mut lists: Vec<Option<Box<ListNode>>>,
    ) -> Option<Box<ListNode>> {
        // Min Heap: (node value, list index)
        let mut min_heap = BinaryHeap::new();

        // Har non-empty list ka first node heap mein add karo.
        for (index, list) in lists.iter().enumerate() {
            if let Some(node) = list {
                min_heap.push(Reverse((node.val, index)));
            }
        }

        // Result list ke liye dummy node.
        let mut dummy = Box::new(ListNode::new(0));

        // Tail result list ke last node ko point karega.
        let mut tail = &mut dummy;

        while let Some(Reverse((_value, index))) = min_heap.pop() {
            // Us list ka smallest node nikalo.
            let mut node = lists[index].take().unwrap();

            // Usi list ko next node par move karo.
            lists[index] = node.next.take();

            // Smallest node ko result list ke end mein add karo.
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();

            // Agar us list mein next node available hai,
            // to usay heap mein add karo.
            if let Some(next_node) = &lists[index] {
                min_heap.push(Reverse((next_node.val, index)));
            }
        }

        // Dummy node ko skip karke actual result return karo.
        dummy.next
    }
}

// Vector se linked list create karne ka function.
fn create_linked_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;

    for value in values.into_iter().rev() {
        let mut node = Box::new(ListNode::new(value));
        node.next = head;
        head = Some(node);
    }

    head
}

// Linked list print karne ka function.
fn print_linked_list(head: &Option<Box<ListNode>>) {
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

fn main() {
    // Three sorted linked lists.
    let lists = vec![
        create_linked_list(vec![1, 4, 5]),
        create_linked_list(vec![1, 3, 4]),
        create_linked_list(vec![2, 6]),
    ];

    // Merge all lists.
    let result = Solution::merge_k_lists(lists);

    println!("Merged Linked List:");
    print_linked_list(&result);
}
