from typing import List
from unittest import TestCase

class Solution:
    def fourSumCount(self, nums1: List[int], nums2: List[int], nums3: List[int], nums4: List[int]) -> int:
        cnt, sums = 0, {}
        for a in nums1:
            for b in nums2:
                sums[a + b] = sums.get(a + b, 0) + 1
        for a in nums3:
            for b in  nums4:
                cnt += sums.get(- a - b, 0)
        return cnt
        
class FourSumCountTest(TestCase):
    def test_1(self):
        self.assertEqual(Solution().fourSumCount([1,2], [-2,-1], [-1,2], [0,2]), 2)

    def test_2(self):
        self.assertEqual(Solution().fourSumCount([0], [0], [0], [0]), 1)
