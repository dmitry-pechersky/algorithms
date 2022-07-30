from typing import List
from unittest import TestCase

class Solution:
    def minimumTotal(self, triangle: List[List[int]]) -> int:
        n = len(triangle)
        dp = [0] * n
        dp[0] = triangle[0][0]
        for i in range(1, n):
            dp[i] = dp[i - 1] + triangle[i][i]
            for j in range(i - 1, 0, -1):
                dp[j] = triangle[i][j]  + (dp[j - 1]  if dp[j - 1] < dp[j] else dp[j])
            dp[0] += triangle[i][0]
        return min(dp)

class MinimumTotalTest(TestCase):
    def test_1(self) -> None:
        self.assertEqual(Solution().minimumTotal([[2],[3,4],[6,5,7],[4,1,8,3]]), 11)

    def test_2(self) -> None:
        self.assertEqual(Solution().minimumTotal([[-10]]), -10)
