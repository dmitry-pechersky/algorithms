from typing import List
from unittest import TestCase

class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        coins = sorted([coin for coin in coins if coin <= amount])
        m = len(coins)
        if amount == 0:
            return 0
        elif m == 0:
            return -1
        dp = [-1] * (amount + 1)
        dp[0] = 0
        for i in range(1, amount + 1):
            mod_cnt = 0
            for coin in coins:
                if coin <= i:
                    if dp[i - coin] != -1 and (dp[i] == -1 or dp[i - coin] + 1 < dp[i]):
                        dp[i] = dp[i - coin]  + 1
                else:
                    break
                if i % coin == 0:
                    mod_cnt += 1
            if mod_cnt == m:
                if dp[amount % i] != -1:
                    return dp[amount % i] + int(amount / i) * dp[i]
                else:
                    return -1
        return dp[amount]

class CoinChangeTest(TestCase):
    def test_1(self):
        self.assertEqual(3, Solution().coinChange([1,2,5], 11))

    def test_2(self):
        self.assertEqual(-1, Solution().coinChange([2], 3))

    def test_3(self):
        self.assertEqual(0, Solution().coinChange([1], 0))

    def test_4(self):
        self.assertEqual(2, Solution().coinChange([2], 4))
