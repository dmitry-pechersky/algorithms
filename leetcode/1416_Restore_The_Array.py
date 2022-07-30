from unittest import TestCase
from typing import List

class Solution:
    def numberOfArrays(self, s: str, k: int) -> int:
        def rec(i: int, dp: List) -> int:
            if dp[i] is None:
                res = 0
                if s[i] != 0:
                    value = 0
                    for j in range(i, n):
                        value = value * 10 + s[j]
                        if value > k:
                            break
                        res = (res + rec(j + 1, dp))  % (1000000000 + 7)
                dp[i] = res
            return dp[i]

        n = len(s)
        dp = [None] *  (n + 1)
        dp[-1] = 1
        s = [ord(ch) - 48 for ch in s]
        return rec(0, dp) 

class NumberOfArraysTest(TestCase):
    def test_1(self) -> None:
        self.assertEqual(1, Solution().numberOfArrays("1000", k = 10000))
    
    def test_2(self) -> None:
        self.assertEqual(0, Solution().numberOfArrays(s = "1000", k = 10))

    def test_3(self) -> None:
        self.assertEqual(8, Solution().numberOfArrays(s = "1317", k = 2000))
    
    def test_4(self) -> None:
        self.assertEqual(1, Solution().numberOfArrays(s = "2020", k = 30))
