import unittest
from typing import List

class Solution:
    def uniquePathsWithObstacles(self, obstacleGrid: List[List[int]]) -> int:
        n, m = len(obstacleGrid), len(obstacleGrid[0])
        dp = [0] * m
        dp[0] = 1
        for i in range(n):
            for j in range(m):
                if obstacleGrid[i][j] == 0:
                    if j > 0:
                        dp[j] += dp[j - 1]
                else:
                    dp[j] = 0
        return dp[m - 1]

class TestUniquePathsWithObstacles(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().uniquePathsWithObstacles([[0,0,0],[0,1,0],[0,0,0]]), 2)

    def test_2(self):
        self.assertEqual(Solution().uniquePathsWithObstacles([[0,1],[0,0]]), 1)
