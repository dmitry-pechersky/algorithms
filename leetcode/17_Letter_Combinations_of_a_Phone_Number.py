import unittest
from typing import List

class Solution:
    def _rec(self, digits, n, pos, letters, mapping, combinations):
        for ch in mapping[digits[pos]]:
            letters.append(ch)
            if pos < n - 1:
                self._rec(digits, n, pos + 1, letters, mapping, combinations)
            else:
                combinations.append("".join(letters))
            letters.pop()

    def letterCombinations(self, digits: str) -> List[str]:
        n = len(digits)
        mapping = {"2": "abc", "3": "def", "4": "ghi", "5": "jkl", "6": "mno", "7": "pqrs", "8": "tuv", "9": "wxyz"}
        combinations = []
        if n > 0:
            self._rec(digits, n, 0, [], mapping, combinations)
        return combinations

class TestLetterCombinations(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().letterCombinations(""), [])    

    def test_2(self):
        self.assertEqual(Solution().letterCombinations("2"), ["a", "b", "c"])            

    def test_3(self):
        self.assertEqual(Solution().letterCombinations("23"), ["ad","ae","af","bd","be","bf","cd","ce","cf"])                    
