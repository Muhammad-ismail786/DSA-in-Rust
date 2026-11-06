# 232. Implement Queue using Stacks (Rust)

## Intuition

A Queue follows FIFO (First In, First Out), which means the element inserted first must be removed first.

A Stack follows LIFO (Last In, First Out), which means the last inserted element is removed first.

We can implement a Queue using two stacks (`s1` and `s2`).

- `s1` is used to insert new elements.
- `s2` is used to access and remove the front element.

When `s2` is empty, we transfer all elements from `s1` to `s2`. This reverses their order and gives us the correct FIFO behavior.

In Rust, we use `Vec<i32>` as a stack.

## Approach

1. **push(x):**
   - Insert the element into `s1` using `push()`.

2. **pop():**
   - If `s2` is empty, transfer all elements from `s1` to `s2`.
   - Remove and return the last element of `s2`.

3. **peek():**
   - If `s2` is empty, transfer all elements from `s1` to `s2`.
   - Return the last element of `s2` without removing it.

4. **empty():**
   - Return true only when both stacks are empty.

## Complexity

- **Time complexity:**
  - `push()`: O(1) amortized
  - `pop()`: O(1) amortized
  - `peek()`: O(1) amortized
  - `empty()`: O(1)
  - Overall: O(n) for n operations.

- **Space complexity:** O(n), where n is the number of elements stored in the Queue.

## Code

See `src/Stack/232-implement-queue-using-stacks.rs`.

## Example

**Input:**
```text
Push(10)
Push(20)
Push(30)
Peek()
Pop()
Peek()
Empty()
