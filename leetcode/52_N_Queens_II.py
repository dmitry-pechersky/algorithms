import unittest
from typing import List

class Solution:
    def totalNQueens(self, n: int) -> int:
        res = 0
        def _rec(n: int , i: int, col_used: List[int], right_diag_used: List[int], left_diag_used: List[int]):
            nonlocal res
            if i >= n:
                res += 1
            else:
                for j in range(n):
                    if not (col_used[j] or right_diag_used[j + i] or left_diag_used[j - i]):
                        col_used[j], right_diag_used[j + i], left_diag_used[j - i] = True, True, True
                        _rec(n, i + 1, col_used, right_diag_used,left_diag_used)
                        col_used[j], right_diag_used[j + i], left_diag_used[j - i] = False, False, False

        _rec(n, 0, [False] * n, [False] * (2 * n), [False] * (2 * n))
        return res

class TestTotalNQueens(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().totalNQueens(4), 2)

    def test_2(self):
        self.assertEqual(Solution().totalNQueens(1), 1)
