# LeetCode 51 - N-Queens (Rust)

## # Intuition

The idea is to place `n` queens on an `n x n` chessboard so that no two queens can attack each other.

A queen can attack horizontally, vertically, and diagonally. Therefore, every queen must be placed in a safe position.

We place one queen in each row and try different columns until we find all valid board configurations.

## # Approach

We use **Backtracking** to solve this problem.

1. Create an `n x n` chessboard filled with `.` (empty cells).
2. Start placing queens from the first row.
3. Use the `is_safe()` function to check whether a queen can be placed in a particular column.
4. Check the upper column, upper-left diagonal, and upper-right diagonal.
5. If the position is safe, place the queen (`Q`) and move to the next row.
6. If all queens are placed successfully, convert the board into strings and save it in the answer.
7. Remove the queen (`.`) and try another column. This process is called backtracking.
8. Return all valid board configurations.

## # Complexity

- **Time complexity:** `O(N! × N)` — We explore possible queen placements and check whether each position is safe.

- **Space complexity:** `O(N²)` — To store the chessboard and recursion-related space, excluding the output.

## # Code

```rust
impl Solution {

    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {

        fn is_safe(
            board: &Vec<Vec<char>>,
            row: usize,
            col: usize,
            n: usize,
        ) -> bool {

            for i in 0..row {
                if board[i][col] == 'Q' {
                    return false;
                }
            }

            let mut i = row as isize - 1;
            let mut j = col as isize - 1;

            while i >= 0 && j >= 0 {
                if board[i as usize][j as usize] == 'Q' {
                    return false;
                }

                i -= 1;
                j -= 1;
            }

            i = row as isize - 1;
            j = col as isize + 1;

            while i >= 0 && j < n as isize {
                if board[i as usize][j as usize] == 'Q' {
                    return false;
                }

                i -= 1;
                j += 1;
            }

            true
        }

        fn backtrack(
            board: &mut Vec<Vec<char>>,
            row: usize,
            n: usize,
            ans: &mut Vec<Vec<String>>,
        ) {

            if row == n {
                let solution: Vec<String> = board
                    .iter()
                    .map(|r| r.iter().collect())
                    .collect();

                ans.push(solution);
                return;
            }

            for col in 0..n {
                if is_safe(board, row, col, n) {
                    board[row][col] = 'Q';

                    backtrack(board, row + 1, n, ans);

                    board[row][col] = '.';
                }
            }
        }

        let n = n as usize;
        let mut board = vec![vec!['.'; n]; n];
        let mut ans = Vec::new();

        backtrack(&mut board, 0, n, &mut ans);

        ans
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
[
    [".Q..","...Q","Q...","..Q."],
    ["..Q.","Q...","...Q",".Q.."]
]
```

There are two distinct solutions for `n = 4`.
