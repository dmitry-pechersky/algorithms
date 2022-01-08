import unittest
from typing import List

class Solution:
    def _rec(self, n: int, left_n: int, right_n: int, combination: List[str], combinations: List[str]):
        if left_n == n and  right_n == n:
            combinations.append("".join(combination))
        else:
            if left_n < n:
                combination.append('(')
                self._rec(n, left_n + 1, right_n, combination, combinations)
                combination.pop()
            if right_n < left_n:
                combination.append(')')
                self._rec(n, left_n, right_n + 1, combination, combinations)
                combination.pop()
        
    def generateParenthesis(self, n: int) -> List[str]:
        combination = []
        combinations = []
        self._rec(n, 0, 0, combination, combinations)
        return combinations

class TestGenerateParenthesis(unittest.TestCase):
    def test_1(self):
        self.assertEqual(set(Solution().generateParenthesis(1)), {"()"})    

    def test_2(self):
        self.assertEqual(set(Solution().generateParenthesis(3)), {"((()))","(()())","(())()","()(())","()()()"})    
