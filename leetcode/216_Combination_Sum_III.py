from typing import List
from unittest import TestCase

class Solution:
    def combinationSum3(self, k: int, n: int) -> List[List[int]]:
        def rec(start, k, n, combination: List[int], combinations: List[List[int]]):
            if k == 0 and n == 0:
                combinations.append(combination.copy())
            elif k > 0 and n > 0:                
                for i in range(start, 10):
                    combination.append(i)
                    rec(i + 1, k - 1, n - i, combination, combinations)
                    combination.pop()

        combination: List[int] = []
        combinations: List[List[int]] = []
        rec(1, k, n, combination, combinations)
        return combinations

class CombinationSum3Test(TestCase):
    
    def test_1(self):
        self.assertEqual([[1,2,4]], Solution().combinationSum3(3, 7))

    def test_2(self):
        self.assertEqual(sorted([[1,2,6],[1,3,5],[2,3,4]]), Solution().combinationSum3(3, 9))        

    def test_3(self):
        self.assertEqual([] , Solution().combinationSum3(4, 1))  
    
              
    def test_4(self):
        self.assertEqual([[5]] , Solution().combinationSum3(1, 5))  
