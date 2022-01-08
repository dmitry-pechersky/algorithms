import unittest
from typing import List

class Solution:
    def _rec(self, candidates: List[int], target: int, pos: int, total: int, combination: List[int], combinations: List[List[int]]):
        if total == target:
            combinations.append(combination.copy())
        if total + candidates[pos] <= target:
            for i in range(pos, len(candidates)):
                combination.append(candidates[i])
                self._rec(candidates, target, i, total + candidates[i], combination, combinations)
                combination.pop()

    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        candidates.sort()
        combination, combinations = [], []
        self._rec(candidates, target, 0, 0, combination, combinations)
        return combinations

class TestCombinationSum(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().combinationSum([2,3,6,7], 7), [[2,2,3],[7]])    

    def test_2(self):
        self.assertEqual(Solution().combinationSum([2,3,5], 8), [[2,2,2,2], [2,3,3], [3,5]])    

    def test_3(self):
        self.assertEqual(Solution().combinationSum([2], 1), [])    

    def test_4(self):
        self.assertEqual(Solution().combinationSum([1], 1), [[1]])    

    def test_5(self):
        self.assertEqual(Solution().combinationSum([1], 2), [[1, 1]])    
