class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        # Arrays of 9 integers. Each integer acts as a 9-bit tracker.
        rows = [0] * 9
        cols = [0] * 9
        boxes = [0] * 9
        
        for r in range(9):
            for c in range(9):
                if board[r][c] == '.':
                    continue
                    
                # Convert string '1'-'9' to an integer 1-9
                val = int(board[r][c])
                
                # Create a bitmask for this number. 
                # e.g., if val is 3, '1 << 3' is binary 000001000
                bit = 1 << val
                
                # Calculate which of the 9 boxes we are in (0 through 8)
                box_idx = (r // 3) * 3 + (c // 3)
                
                # Use Bitwise AND (&). If the result is not 0, the bit was already set!
                if (rows[r] & bit) or (cols[c] & bit) or (boxes[box_idx] & bit):
                    return False
                    
                # Use Bitwise OR (|=) to set the bit, marking the number as seen
                rows[r] |= bit
                cols[c] |= bit
                boxes[box_idx] |= bit
                
        return True