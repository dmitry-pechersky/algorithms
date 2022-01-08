import unittest
from typing import List

class Solution:
    def canJump(self, nums: List[int]) -> bool:
        n = len(nums)
        max_jump = 0
        for i in range(n):
            if i > max_jump:
                return False
            if i + nums[i] > max_jump:
                max_jump = i + nums[i]
                if max_jump >= n - 1:
                    break
        return True 

class TestCanJump(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().canJump([2,3,1,1,4]), True)

    def test_2(self):
        self.assertEqual(Solution().canJump([3,2,1,0,4]), False)
