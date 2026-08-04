class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        # here since we have a constraint of 1-9 we could use bytes and avoid creating
        # hashmap
        # make set of each row and col
        for row in board:  
            # create filtered array to filter out '.' and change to int maybe
            digits = [c for c in row if c != '.']
            if len(digits) != len(set(digits)):
                return False
            
        for c in range(len(board)):
            col_digs = [board[r][c] for r in range(len(board)) if board[r][c] != '.']
            if len(col_digs) != len(set(col_digs)):
                return False
        
        for r_start in range(0, 9, 3):
            for c_start in range(0, 9, 3):
                box_digits = [
                    board[r][c]
                    for r in range(r_start, r_start + 3)
                    for c in range(c_start, c_start + 3)
                    if board[r][c] != '.'
                ]
                if len(box_digits) != len(set(box_digits)):
                    return False
        return True

            
