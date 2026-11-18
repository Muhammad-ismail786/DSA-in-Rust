
# 37. Sudoku Solver

**Difficulty:** Hard

**LeetCode:** https://leetcode.com/problems/sudoku-solver/

## Intuition

We need to fill all empty cells ('.') in a 9x9 Sudoku board while following three rules:

- Each number from 1 to 9 must appear exactly once in every row.
- Each number from 1 to 9 must appear exactly once in every column.
- Each number from 1 to 9 must appear exactly once in every 3x3 box.

We can use Backtracking with Recursion to try different numbers until we find a valid solution.

## Approach

1. Traverse the entire board using two nested loops.
2. Find an empty cell represented by '.'.
3. Try numbers from '1' to '9'.
4. Check whether the number already exists in the current row, column, or 3x3 box.
5. If the number is valid, place it in the empty cell.
6. Recursively solve the remaining Sudoku.
7. If recursion returns true, the Sudoku is solved, so return true.
8. If the number does not lead to a solution, reset the cell to '.' and try another number. This process is called Backtracking.
9. If no empty cells remain, return true because the Sudoku is solved.

## Complexity

- Time complexity: O(9^m), where m is the number of empty cells (upper bound).
- Space complexity: O(m), where m is the number of empty cells due to the recursion stack.

## Code

```rust
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {

        fn backtrack(board: &mut Vec<Vec<char>>) -> bool {

            for row in 0..9 {

                for col in 0..9 {

                    if board[row][col] == '.' {

                        for num in '1'..='9' {

                            let mut valid = true;

                            for i in 0..9 {

                                if board[row][i] == num ||
                                   board[i][col] == num {

                                    valid = false;
                                    break;
                                }

                                let r = 3 * (row / 3) + i / 3;
                                let c = 3 * (col / 3) + i % 3;

                                if board[r][c] == num {
                                    valid = false;
                                    break;
                                }
                            }

                            if valid {

                                board[row][col] = num;

                                if backtrack(board) {
                                    return true;
                                }

                                board[row][col] = '.';
                            }
                        }

                        return false;
                    }
                }
            }

            true
        }

        backtrack(board);
    }
}
```
