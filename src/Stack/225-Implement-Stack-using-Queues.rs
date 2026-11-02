
use std::collections::VecDeque;

struct MyStack {
    q1: VecDeque<i32>,
    q2: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self {
            q1: VecDeque::new(),
            q2: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.q2.push_back(x);

        while !self.q1.is_empty() {
            let value = self.q1.pop_front().unwrap();
            self.q2.push_back(value);
        }

        std::mem::swap(&mut self.q1, &mut self.q2);
    }

    fn pop(&mut self) -> i32 {
        self.q1.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.q1.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.q1.is_empty()
    }
}

fn main() {
    let mut stack = MyStack::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Top: {}", stack.top());

    println!("Pop: {}", stack.pop());

    println!("Top after pop: {}", stack.top());

    println!("Is Empty: {}", stack.empty());

    stack.pop();
    stack.pop();

    println!("Is Empty after removing all elements: {}", stack.empty());
}
