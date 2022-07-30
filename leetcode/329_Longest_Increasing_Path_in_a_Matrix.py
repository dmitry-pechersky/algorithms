from typing import List
from unittest import TestCase

class Solution:
    def longestIncreasingPath(self, matrix: List[List[int]]) -> int:
        n, m = len(matrix), len(matrix[0])
        dp = [[0] * m for i in range(n)]

        def rec(i, j, dp):
            if dp[i][j] == 0:
                max_next_lengh = 0
                for i1, j1 in ((i + 1, j), (i, j + 1), (i - 1, j), (i, j - 1)):
                    if 0 <= i1 < n and 0 <= j1 < m and matrix[i][j] < matrix[i1][j1]:
                        next_lengh = rec(i1, j1, dp)
                        if next_lengh > max_next_lengh:
                            max_next_lengh = next_lengh
                dp[i][j] = max_next_lengh + 1
            return dp[i][j]

        max_length = 0
        for i in range(n):
            for j in range(m):
                if dp[i][j] == 0:
                    rec(i, j, dp)
                if dp[i][j] > max_length:
                    max_length = dp[i][j]
        return max_length

class LongestIncreasingPathTest(TestCase):
    def test_1(self):
        self.assertEqual(4, Solution().longestIncreasingPath([[9,9,4],[6,6,8],[2,1,1]]))

    def test_2(self):
        self.assertEqual(4, Solution().longestIncreasingPath([[3,4,5],[3,2,6],[2,2,1]]))        

    def test_3(self):
        self.assertEqual(1, Solution().longestIncreasingPath([[1]]))                