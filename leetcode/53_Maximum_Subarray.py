import unittest
from typing import List

class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        max_sum = nums[0]
        current_sum = 0
        for num in nums:
            current_sum += num
            if current_sum > max_sum:
                max_sum = current_sum
            if current_sum < 0:
                current_sum = 0
        return max_sum



class TestMaxSubArray(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().maxSubArray([-2,1,-3,4,-1,2,1,-5,4]), 6)

    def test_2(self):
        self.assertEqual(Solution().maxSubArray([1]), 1)

    def test_3(self):
        self.assertEqual(Solution().maxSubArray([5,4,-1,7,8]), 23)

