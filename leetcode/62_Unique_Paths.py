import unittest

class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        grid = [[1] * n for i in range(m)]
        for i in range(1, m):
            for j in range(1,n):
                grid[i][j] = grid[i - 1][j] + grid[i][j - 1]
        return grid[m - 1][n - 1]

class TestUniquePaths(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().uniquePaths(3, 7), 28)

    def test_2(self):
        self.assertEqual(Solution().uniquePaths(3, 2), 3)

    def test_3(self):
        self.assertEqual(Solution().uniquePaths(7, 3), 28)

    def test_4(self):
        self.assertEqual(Solution().uniquePaths(3, 3), 6)
