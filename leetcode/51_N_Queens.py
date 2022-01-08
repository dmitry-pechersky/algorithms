import unittest
from typing import List

class Solution:
    def solveNQueens(self, n: int) -> List[List[str]]:
        boards = []
    def _rec(self, n: int , i: int, board: List[List[int]], boards: List[List[str]], col_used: List[int], right_diag_used: List[int], left_diag_used: List[int]):
        if i >= n:
            boards.append(["".join(row) for row in board])
        else:
            for j in range(n):
                if not (col_used[j] or right_diag_used[j + i] or left_diag_used[j - i]):
                    col_used[j], right_diag_used[j + i], left_diag_used[j - i] = True, True, True
                    board[i][j] = 'Q'
                    self._rec(n, i + 1, board, boards, col_used, right_diag_used,left_diag_used)
                    board[i][j] = '.'
                    col_used[j], right_diag_used[j + i], left_diag_used[j - i] = False, False, False
        self._rec(n, 0, [['.'] * n for i in range(n)], boards, [False] * n, [False] * (2 * n), [False] * (2 * n))
        return boards

class TestSolveNQueens(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().solveNQueens(4), [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]])

    def test_2(self):
        self.assertEqual(len(Solution().solveNQueens(245)), [["Q"]])
            

