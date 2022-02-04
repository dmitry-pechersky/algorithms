from unittest import TestCase
from math import sqrt

class Solution:
    def winnerSquareGame(self, n: int) -> bool:
        dp = [False] * (n + 1)
        squares = [i * i for i in range(1, int(sqrt(n)) + 1)]
        for i in range(1, n + 1):
            for square in  squares:
                if square > i:
                    break
                elif not dp[i - square]:
                    dp[i] = True
                    break
        return dp[n]

class WinnerSquareGame(TestCase):
    def test_1(self):
        self.assertTrue(Solution().winnerSquareGame(1))

    def test_2(self):
        self.assertFalse(Solution().winnerSquareGame(2))

    def test_3(self):
        self.assertTrue(Solution().winnerSquareGame(4))                