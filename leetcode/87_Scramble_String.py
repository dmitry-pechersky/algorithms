import unittest
from typing import Dict, Tuple

class Solution:
    def isScramble(self, s1: str, s2: str) -> bool:
        def _rec(i1: int, i2: int, j: int, dp: Dict[Tuple[int, int, int], bool]) -> bool:
            if i1 == i2:
                return s1[i1] == s2[j]
            if (i1, i2, j) not in dp:
                res = False
                for k in range(i1, i2):
                    if _rec(i1, k, j, dp) and _rec(k + 1, i2, j + k - i1 + 1, dp):
                        res = True
                        break
                    if _rec(i1, k, j + i2 - k, dp) and _rec(k + 1, i2, j, dp):
                        res = True
                        break
                dp[(i1, i2, j)] = res
            return dp[(i1, i2, j)]
        return _rec(0, len(s1) - 1, 0, dict())

class IsScrambleTest(unittest.TestCase):
    def test_1(self) -> None:
        self.assertFalse(Solution().isScramble("abcde", "caebd"))

    def test_2(self) -> None:
        self.assertTrue(Solution().isScramble("great", "rgeat"))
    
    def test_3(self) -> None:
        self.assertTrue(Solution().isScramble("a", "a"))

    def test_4(self) -> None:
        self.assertTrue(Solution().isScramble("ab", "ba"))
