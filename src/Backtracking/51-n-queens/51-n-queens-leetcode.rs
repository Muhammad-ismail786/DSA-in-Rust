
impl Solution {

    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {

        fn is_safe(
            board: &Vec<Vec<char>>,
            row: usize,
            col: usize,
            n: usize,
        ) -> bool {

            // Check upper column
            for i in 0..row {
                if board[i][col] == 'Q' {
                    return false;
                }
            }

            // Check upper-left diagonal
            let mut i = row as isize - 1;
            let mut j = col as isize - 1;

            while i >= 0 && j >= 0 {

                if board[i as usize][j as usize] == 'Q' {
                    return false;
                }

                i -= 1;
                j -= 1;
            }

            // Check upper-right diagonal
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

            // All queens placed successfully
            if row == n {

                let solution: Vec<String> = board
                    .iter()
                    .map(|r| r.iter().collect())
                    .collect();

                ans.push(solution);

                return;
            }

            // Try every column
            for col in 0..n {

                if is_safe(board, row, col, n) {

                    // Place queen
                    board[row][col] = 'Q';

                    // Move to next row
                    backtrack(board, row + 1, n, ans);

                    // Backtrack: remove queen
                    board[row][col] = '.';
                }
            }
        }

        let n = n as usize;

        let mut board = vec![vec!['.'; n]; n];

        let mut ans: Vec<Vec<String>> = Vec::new();

        backtrack(&mut board, 0, n, &mut ans);

        ans
    }
}
