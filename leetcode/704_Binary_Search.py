from typing import List
from unittest import TestCase

class Solution:
    def search(self, nums: List[int], target: int) -> int:
        start, end = 0, len(nums) - 1
        while start <= end:
            middle = int((start + end) / 2)
            if nums[middle] == target:
                return middle
            if target < nums[middle]:
                end = middle - 1
            else:
                start = middle + 1
        return -1 

class SearchTest(TestCase):
    def test_1(self) -> None:
        self.assertEqual(Solution().search([-1,0,3,5,9,12], 9), 4)

    def test_2(self) -> None:
        self.assertEqual(Solution().search([-1,0,3,5,9,12], 2), -1)

    def test_3(self) -> None:
        self.assertEqual(Solution().search([1,100], 50), -1)

    def test_4(self) -> None:
        self.assertEqual(Solution().search([5], 5), 0)
