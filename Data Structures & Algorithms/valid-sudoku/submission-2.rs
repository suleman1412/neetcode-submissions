impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // Use fixed-size arrays of 16-bit integers (stack allocated, zero overhead)
        let mut rows = [0u16; 9];
        let mut cols = [0u16; 9];
        let mut boxes = [0u16; 9];

        for r in 0..9 {
            for c in 0..9 {
                let val = board[r][c];
                
                if val == '.' {
                    continue;
                }

                // Convert char '1'-'9' to an index 0-8 using ASCII byte math
                // This is a zero-cost operation in Rust, unlike Python's int()
                let bit_idx = val as u8 - b'1';
                let bit = 1 << bit_idx;
                
                let box_idx = (r / 3) * 3 + (c / 3);

                // Check if the bit is already set (meaning we've seen the number)
                if (rows[r] & bit) != 0 || (cols[c] & bit) != 0 || (boxes[box_idx] & bit) != 0 {
                    return false;
                }

                // Set the bit to mark the number as seen
                rows[r] |= bit;
                cols[c] |= bit;
                boxes[box_idx] |= bit;
            }
        }
        
        true
    }
}