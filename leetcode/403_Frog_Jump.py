from unittest import TestCase
from typing import List

class Solution:
    def canCross(self, stones: List[int]) -> bool:
        hieghts = set(stones)
        target = stones[-1]
        dic = {}

        def rec(h: int, k: int):
            next_h = h + k
            if next_h == target:
                return True
            if (h, k) not in dic:
                dic[(h, k)] = False
                if next_h in hieghts:
                    for next_k in (k + 1, k, k - 1):
                        if rec(next_h, next_k):
                            dic[(h, k)] = True
                            break
            return dic[(h, k)]

        return rec(0, 1)
            

class CanCrossTest(TestCase):
    def test_1(self):
        self.assertTrue(Solution().canCross([0,1,3,5,6,8,12,17]))

    def test_2(self):
        self.assertFalse(Solution().canCross([0,1,2,3,4,8,9,11]))        