use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let len = board.len();

        // Helper function taking a slice reference to avoid cloning
        fn is_valid_group(arr: &[char]) -> bool {
            let unique: HashSet<&char> = arr.iter().collect();
            arr.len() == unique.len()
        }

        // 1. Check Rows
        for row in &board {
            let digits: Vec<char> = row.iter().cloned().filter(|&c| c != '.').collect();
            if !is_valid_group(&digits) {
                return false;
            }
        }

        // 2. Check Columns
        for c in 0..len {
            let col_digs: Vec<char> = (0..len)
                .map(|r| board[r][c])
                .filter(|&c| c != '.')
                .collect();
            if !is_valid_group(&col_digs) {
                return false;
            }
        }

        // 3. Check 3x3 Boxes
        for r_start in (0..9).step_by(3) {
            for c_start in (0..9).step_by(3) {
                let mut box_digits = Vec::new();
                for r in r_start..r_start + 3 {
                    for c in c_start..c_start + 3 {
                        if board[r][c] != '.' {
                            box_digits.push(board[r][c]);
                        }
                    }
                }
                if !is_valid_group(&box_digits) {
                    return false;
                }
            }
        }

        true
    }
}