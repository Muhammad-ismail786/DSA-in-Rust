struct MyQueue {
    s1: Vec<i32>,
    s2: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            s1: Vec::new(),
            s2: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.s1.push(x);
    }

    fn transfer(&mut self) {
        if self.s2.is_empty() {
            while let Some(value) = self.s1.pop() {
                self.s2.push(value);
            }
        }
    }

    fn pop(&mut self) -> i32 {
        self.transfer();
        self.s2.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        self.transfer();
        *self.s2.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.s1.is_empty() && self.s2.is_empty()
    }
}

fn main() {
    let mut queue = MyQueue::new();

    queue.push(10);
    queue.push(20);
    queue.push(30);

    println!("Front element: {}", queue.peek());
    println!("Removed element: {}", queue.pop());
    println!("Front after Pop: {}", queue.peek());
    println!("Is Queue empty? {}", queue.empty());
}
