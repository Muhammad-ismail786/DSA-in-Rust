
# 225. Implement Stack using Queues

## Problem

Implement a Last-In-First-Out (LIFO) stack using only two queues.

The stack should support these operations:

- `push(x)`: Push element x onto the stack.
- `pop()`: Remove and return the top element.
- `top()`: Return the top element.
- `empty()`: Return true if the stack is empty.

## Intuition

A Stack follows LIFO (Last In, First Out).

This means the element inserted last will be removed first.

A Queue follows FIFO (First In, First Out).

This means the element inserted first will be removed first.

We use two queues (`q1` and `q2`) to make a queue behave like a stack.

Whenever we insert a new element:

1. Insert the new element into `q2`.
2. Move all elements from `q1` into `q2`.
3. Swap `q1` and `q2`.

After these steps, the newest element always stays at the front of `q1`.

## Approach

We use Rust's `VecDeque` to implement both queues.

### 1. push(x)

- Insert the new element into `q2`.
- Move all elements from `q1` to `q2`.
- Swap both queues using `std::mem::swap()`.

### 2. pop()

- Remove and return the front element of `q1`.

### 3. top()

- Return the front element of `q1` without removing it.

### 4. empty()

- Check whether `q1` is empty.

## Complexity Analysis

| Operation | Time Complexity |
|---|---|
| push() | O(n) |
| pop() | O(1) |
| top() | O(1) |
| empty() | O(1) |

**Space Complexity:** O(n)

We use two queues to store the stack elements.

## Code (Rust)

```rust
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
```
