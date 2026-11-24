
# 52. N-Queens II

**Difficulty:** Hard

**Topics:** Backtracking, Recursion, Arrays

**Language:** Rust

## # Intuition

The main idea is to place `n` queens on an `n x n` chessboard so that no two queens attack each other.

We place one queen in each row and check whether its column and both diagonals are already occupied.

If a safe position is found, we place the queen and move to the next row using recursion.

Whenever all `n` rows have a queen, we increase the count by 1.

## # Approach

1. Create three boolean vectors:
   - `cols` to track occupied columns.
   - `diag1` to track occupied diagonals using `row + n - 1 - col`.
   - `diag2` to track occupied diagonals using `row + col`.

2. Start placing queens from row 0.

3. Check every column to find a safe position.

4. If the position is safe:
   - Mark the column and both diagonals as occupied.
   - Recursively move to the next row.
   - Backtrack by marking them as unoccupied.

5. If `row == n`, all queens are successfully placed, so increment `count`.

6. Finally, return the total count.

## # Complexity

- Time complexity: O(N!) approximately, due to backtracking and trying different queen placements.

- Space complexity: O(N) auxiliary space for the three boolean vectors, excluding the recursion stack.

## # Code

```rust
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {

        let n = n as usize;

        let mut count = 0;

        let mut cols = vec![false; n];
        let mut diag1 = vec![false; 2 * n - 1];
        let mut diag2 = vec![false; 2 * n - 1];

        fn solve(
            row: usize,
            n: usize,
            count: &mut i32,
            cols: &mut Vec<bool>,
            diag1: &mut Vec<bool>,
            diag2: &mut Vec<bool>,
        ) {

            if row == n {
                *count += 1;
                return;
            }

            for col in 0..n {

                let d1 = row + n - 1 - col;
                let d2 = row + col;

                if cols[col] || diag1[d1] || diag2[d2] {
                    continue;
                }

                cols[col] = true;
                diag1[d1] = true;
                diag2[d2] = true;

                solve(row + 1, n, count, cols, diag1, diag2);

                cols[col] = false;
                diag1[d1] = false;
                diag2[d2] = false;
            }
        }

        solve(
            0,
            n,
            &mut count,
            &mut cols,
            &mut diag1,
            &mut diag2,
        );

        count
    }
}
```

## # Example

**Input:**
```text
n = 4
```

**Output:**
```text
2
```

**Explanation:**

There are two distinct ways to place four queens on a 4 x 4 chessboard so that no two queens attack each other.

## # Tags

- Backtracking
- Recursion
- Arrays
- LeetCode
```
