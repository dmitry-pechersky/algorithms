from unittest import TestCase
from typing import List

class Solution:
    def findMaxLength(self, nums: List[int]) -> int:
        res, s, dic = 0, 0, {0:0}
        for i, num in enumerate(nums, 1):
            s += 1 if num == 0 else -1
            if s not in dic:
                dic[s] = i
            else:
                if i - dic[s] > res:
                    res = i - dic[s]
        return res
            

class FindMaxLengthTest(TestCase):
    def test_1(self) -> None:
        self.assertEqual(Solution().findMaxLength([0,1]), 2)

    def test_2(self) -> None:
        self.assertEqual(Solution().findMaxLength([0,1,0]), 2)        