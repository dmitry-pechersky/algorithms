from unittest import TestCase
from typing import List

class Solution:
    def minSwap(self, nums1: List[int], nums2: List[int]) -> int:
        n = len(nums1)
        dp0, dp1 = 0, 1
        for i in range(1, n):
            cur_dp0, cur_dp1 = n, n
            if nums1[i - 1] <  nums1[i] and nums2[i - 1] <  nums2[i]:
                cur_dp0 =  dp0
                cur_dp1 = dp1 + 1
            if nums2[i - 1] <  nums1[i] and nums1[i - 1] <  nums2[i]:
                if dp1 < cur_dp0:
                    cur_dp0 = dp1
                if dp0 + 1 < cur_dp1:
                    cur_dp1 = dp0 + 1
            dp0, dp1 = cur_dp0, cur_dp1
        return min(dp0, dp1)

class MinSwapTest(TestCase):
    def test_1(self):
        self.assertEqual(1, Solution().minSwap([1,3,5,4], [1,2,3,7]))

    def test_2(self):
        self.assertEqual(1, Solution().minSwap([0,3,5,8,9], [2,1,4,6,9]))

           