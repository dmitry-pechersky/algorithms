import unittest
from typing import List

class Solution:
    def combine(self, n: int, k: int) -> List[List[int]]:
        combinations = []
        def rec(i: int, n: int, k: int, combination: List[int] ,combinations: List[List[int]]) -> None:
            if k == 0:
                combinations.append(combination.copy())
            elif n - i + 1 >= k:
                rec(i + 1, n, k, combination, combinations)
                combination.append(i)
                rec(i + 1, n, k - 1, combination, combinations)
                combination.pop()
        rec(1, n, k, [], combinations)
        return combinations

class TestCombine(unittest.TestCase):
    def test_1(self):
        self.assertEqual(sorted(Solution().combine(4, 2)), sorted([[2,4], [3,4], [2,3], [1,2], [1,3], [1,4]]))    

    def test_2(self):
        self.assertEqual(Solution().combine(1, 1), [[1]])            