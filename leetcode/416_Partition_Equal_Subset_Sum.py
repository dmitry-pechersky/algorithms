import unittest
from typing import List

class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        total_sum = sum(nums)
        if total_sum % 2 != 0:
            return False
        target = total_sum / 2
        sums = set([0])
        for num in nums:
            sums.update(set(s + num for s in sums if s + num <= target))
            if target in sums:
                return True
        return False

class TestCanPartition(unittest.TestCase):
    def test_1(self) -> None:
        self.assertTrue(Solution().canPartition([1,5,11,5]))        

    def test_2(self) -> None:
        self.assertFalse(Solution().canPartition([1,2,3,5]))
