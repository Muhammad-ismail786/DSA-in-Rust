
struct Solution;

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

fn main() {

    let mut board = vec![
        vec!['5','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9'],
    ];

    Solution::solve_sudoku(&mut board);

    println!("Solved Sudoku:\n");

    for row in &board {

        for cell in row {
            print!("{} ", cell);
        }

        println!();
    }
}
