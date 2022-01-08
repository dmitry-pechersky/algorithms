import unittest
from typing import List

class Solution:
    def _rec(self, candidates: List[int], target: int, pos: int, total: int, combination: List[int], combinations: List[List[int]]):
        if total == target:
            combinations.append(combination.copy())
        if total < target:
            for i in range(pos, len(candidates)):
                if i == pos or candidates[i] != candidates[i - 1]:
                    combination.append(candidates[i])
                    self._rec(candidates, target, i + 1, total + candidates[i], combination, combinations)
                    combination.pop()

    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        candidates.sort()
        combination, combinations = [], []
        self._rec(candidates, target, 0, 0, combination, combinations)
        return combinations


class TestCombinationSum2(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().combinationSum2([10,1,2,7,6,1,5], 8), [[1,1,6], [1,2,5], [1,7], [2,6]])

    def test_2(self):
        self.assertEqual(Solution().combinationSum2([2,5,2,1,2], 5), [[1,2,2], [5]])