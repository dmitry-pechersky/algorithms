import unittest
from typing import List

class Solution:
    def _rec(self, n: int, nums: List[int], i: int , combinations: List[List[int]]):
        if i >= n:
            combinations.append(nums.copy())
        else:
            self._rec(n, nums, i + 1, combinations)
            for j in range(i):
                nums[j], nums[i] = nums[i], nums[j]
                self._rec(n, nums, i + 1, combinations)
                nums[j], nums[i] = nums[i], nums[j]

    def permute(self, nums: List[int]) -> List[List[int]]:
        combinations = []
        self._rec(len(nums), nums, 0, combinations)
        return combinations

class TestPermute(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().permute([1,2,3]), [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]])        

    def test_2(self):
        self.assertEqual(Solution().permute([0,1]), [[0,1],[1,0]])

    def test_3(self):
        self.assertEqual(Solution().permute([1]), [[1]])
