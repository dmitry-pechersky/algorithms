from typing import List
from unittest import TestCase

class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        current, cnt = nums[0], 1
        for i in range(len(nums)):
            cnt += 1 if current == nums[i] else -1
            if cnt == 0:
                current = nums[i]
                cnt = 1
        return current

class MajorityElement(TestCase):
    def test_1(self) -> None:
        self.assertEqual(Solution().majorityElement([3,2,3]), 3)

    def test_2(self) -> None:
        self.assertEqual(Solution().majorityElement([2,2,1,1,1,2,2]), 2)
