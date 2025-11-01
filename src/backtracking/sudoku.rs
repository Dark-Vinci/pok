use super::Solution;

impl Solution {
    fn sudoku(mut board: Vec<Vec<char>>) {
        Self::sudoku_helper(&mut board);
    }

    fn sudoku_helper(board: &mut Vec<Vec<char>>) -> bool {
        let (mut row, mut col) = (-1, -1);

        'outer: for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == '.' {
                    (row, col) = (i as i32, j as i32);
                    break 'outer
                }
            }
        }

        if row == -1 && col == -1 {
            return true
        }

        for i in 1..=9u8  {
            let i_byte = i as char;

            if Self::is_valid(&board, i_byte, row, col) {
                board[row as usize][col as usize] = i_byte;

                if Self::sudoku_helper(board) {
                    return true;
                }

                board[row as usize][col as usize] = '.';
            }
        }

        false
    }

    fn is_valid(board: &Vec<Vec<char>>, value: char, row: i32, col: i32) -> bool {
        for i in 0..board[row as usize].len() {
            if board[row as usize][i] == value {
                return false
            }
        }

        for i in 0..board.len() {
            if board[i][col as usize] == value {
                return false
            }
        }

        let (row_start, col_start) = (row - row % 3, col - col % 3);

        for i in row_start..row_start + 3 {
            for j in col_start..col_start + 3 {
                if board[i as usize][j as usize] == value {
                    return false
                }
            }
        }

        true
    }
}