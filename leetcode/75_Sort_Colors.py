import unittest
from typing import List

class Solution:
    def sortColors(self, nums: List[int]) -> None:
        cnts = [0, 0, 0]
        for num in nums:
            cnts[num] += 1
        j = 0
        for i in range(3):
            while cnts[i] > 0:
                nums[j] = i
                cnts[i] -= 1
                j += 1

class TestSortColors(unittest.TestCase):
    def test_1(self):
        nums = [2,0,2,1,1,0]
        Solution().sortColors(nums)
        self.assertEqual(nums, [0,0,1,1,2,2])

    def test_2(self):
        nums = [2,0,1]
        Solution().sortColors(nums)
        self.assertEqual(nums, [0,1,2])

    def test_3(self):
        nums = [0]
        Solution().sortColors(nums)
        self.assertEqual(nums, [0])

    def test_4(self):
        nums = [1]
        Solution().sortColors(nums)
        self.assertEqual(nums, [1])
