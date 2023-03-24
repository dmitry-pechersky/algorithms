import unittest

class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        dp = [1] * n
        for _ in range(1, m):
            for i in range(1,n):
                dp[i] += dp[i - 1]
        return dp[n - 1]

class TestUniquePaths(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().uniquePaths(3, 7), 28)

    def test_2(self):
        self.assertEqual(Solution().uniquePaths(3, 2), 3)

    def test_3(self):
        self.assertEqual(Solution().uniquePaths(7, 3), 28)

    def test_4(self):
        self.assertEqual(Solution().uniquePaths(3, 3), 6)
