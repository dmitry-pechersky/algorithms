import unittest

class Solution:
    def minDistance(self, word1: str, word2: str) -> int:
        n1, n2 = len(word1), len(word2)
        dp = [[0] * (n2 + 1) for i in range(n1 + 1)]
        for i in range(n1 + 1):
            dp[i][0] = i
        for i in range(n2 + 1):
            dp[0][i] = i
        for i in range(1, n1 + 1):
            for j in range(1, n2 + 1):
                dp[i][j] = dp[i - 1][j - 1] + (0 if word1[i - 1] == word2[j - 1] else 1)
                if dp[i][j - 1] + 1 < dp[i][j]:
                    dp[i][j] = dp[i][j - 1] + 1
                if dp[i - 1][j] + 1< dp[i][j]:
                    dp[i][j] = dp[i - 1][j] + 1
        return dp[n1][n2]                

class TestMinDistance(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().minDistance("horse", "ros"), 3)

    def test_2(self):
        self.assertEqual(Solution().minDistance("intention", "execution"), 5)        
    
    def test_3(self):
        self.assertEqual(Solution().minDistance("", "a"), 1)                