from unittest import TestCase

class Solution:
    def numDistinct(self, s: str, t: str) -> int:
        s_n, t_n = len(s), len(t)
        dp = {}
        def rec(i: int, j: int):
            nonlocal dp
            if i <= s_n and j == t_n:
                return 1
            elif i >= s_n or (s_n - i) < (t_n - j):
                return 0
            if (i, j) not in dp:
                dp[(i, j)] = rec(i + 1, j)
                if s[i] == t[j]:
                    dp[(i, j)] += rec(i + 1, j + 1)
            return dp[(i, j)]
        return rec(0, 0)

class NumDistinctTest(TestCase):
    def test_1(self) -> None:
        self.assertEqual(Solution().numDistinct("rabbbit", "rabbit"), 3)

    def test_2(self) -> None:
        self.assertEqual(Solution().numDistinct("babgbag", "bag"), 5)        

    def test_3(self) -> None:
        self.assertEqual(Solution().numDistinct("fff", "ffff"), 0)        
