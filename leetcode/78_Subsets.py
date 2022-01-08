import unittest
from typing import List

class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        def rec(i: int, nums: List[int], combination: List[int], combinations: List[List[int]]) -> None:
            if i >= len(nums):
                combinations.append(combination.copy())
            else:
                rec(i + 1, nums, combination, combinations)
                combination.append(nums[i])
                rec(i + 1, nums, combination, combinations)
                combination.pop()
        combinations = []
        rec(0, nums, [], combinations)
        return combinations

class TestSubsets(unittest.TestCase):
    def test_1(self):
        self.assertEqual(sorted(Solution().subsets([1,2,3])), sorted([[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]))    

    def test_2(self):
        self.assertEqual(Solution().subsets([0]), [[],[0]])    
