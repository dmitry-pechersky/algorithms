import unittest
from typing import List

class Solution:
    def uniquePathsWithObstacles(self, obstacleGrid: List[List[int]]) -> int:
        if obstacleGrid[0][0] == 1:
            return 0
        m, n = len(obstacleGrid), len(obstacleGrid[0])
        grid = [[0] * n for i in range(m)]
        for i in range(n):
            if obstacleGrid[0][i] == 1:
                break
            grid[0][i] = 1
        for i in range(m):
            if obstacleGrid[i][0] == 1:
                break
            grid[i][0] = 1            
        for i in range(1, m):
            for j in range(1, n):
                if obstacleGrid[i][j] == 0:
                    grid[i][j] = grid[i - 1][j] + grid[i][j - 1]
        return grid[m - 1][n - 1]

class TestUniquePathsWithObstacles(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().uniquePathsWithObstacles([[0,0,0],[0,1,0],[0,0,0]]), 2)

    def test_2(self):
        self.assertEqual(Solution().uniquePathsWithObstacles([[0,1],[0,0]]), 1)
