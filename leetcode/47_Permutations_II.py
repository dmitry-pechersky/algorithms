import unittest
from typing import List, Tuple, Set

class Solution:
    def _rec(self, n: int, nums: List[int], i: int, combinations: Set[Tuple[int]]):
        if i >= n:
            tp = tuple(nums)
            if tp not in combinations: 
                combinations.add(tp)
        else:
            self._rec(n, nums, i + 1, combinations)
            for j in range(i):
                if nums[i] != nums[j]:
                    nums[j], nums[i] = nums[i], nums[j]
                    self._rec(n, nums, i + 1, combinations)
                    nums[j], nums[i] = nums[i], nums[j]

    def permuteUnique(self, nums: List[int]) -> Set[Tuple[int]]:
        combinations = set()
        self._rec(len(nums), nums, 0, combinations)
        return combinations

class TestPermuteUnique(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().permuteUnique([1,1,2]), [[1,1,2], [1,2,1], [2,1,1]])        

    def test_2(self):
        self.assertEqual(Solution().permuteUnique([1,2,3]), [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]])
