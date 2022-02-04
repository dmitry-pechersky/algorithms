from typing import List
from unittest import TestCase
from math import gcd

class Solution:
    def rotate(self, nums: List[int], k: int) -> None:        
        n = len(nums)
        k = k % n
        if k > 0:
            for i in range(gcd(n, k)):
                j, tmp = i, nums[i]
                while True:
                    next_j = (j + k) % n
                    nums[next_j], tmp = tmp, nums[next_j]
                    j = next_j
                    if i == j:
                        break

class RotateTest(TestCase):
    def test_1(self):
        nums = [1,2,3,4,5,6,7]
        Solution().rotate(nums, 3)
        self.assertEqual(nums, [5,6,7,1,2,3,4])

    def test_2(self):
        nums = [-1,-100,3,99]
        Solution().rotate(nums, 2)
        self.assertEqual(nums, [3,99,-1,-100])