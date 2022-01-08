import unittest
from typing import List

class Solution:
    def firstMissingPositive(self, nums: List[int]) -> int:      
        n = len(nums)
        for i in range(n):
            while 0 < nums[i] and nums[i] <= n and nums[nums[i] - 1] != nums[i]:
                tmp = nums[nums[i] - 1]
                nums[nums[i] - 1] = nums[i]
                nums[i] = tmp
        for i in range(n):
            if nums[i] != i + 1:
                return i + 1
        return n + 1

class TestFirstMissingPositive(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().firstMissingPositive([1,2,0]), 3)

    def test_2(self):
        self.assertEqual(Solution().firstMissingPositive([3,4,-1,1]), 2)

    def test_3(self):
        self.assertEqual(Solution().firstMissingPositive([7,8,9,11,12]), 1)

    def test_4(self):
        self.assertEqual(Solution().firstMissingPositive([1]), 2)

    def test_5(self):
        self.assertEqual(Solution().firstMissingPositive([1,1]), 2)
