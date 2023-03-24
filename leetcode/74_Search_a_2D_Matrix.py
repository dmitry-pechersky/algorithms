from typing import List
from unittest import TestCase

class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        n, m = len(matrix), len(matrix[0])
        left, right = 0, n * m - 1
        while left <= right:
            middle = (left + right) // 2
            middle_value = matrix[middle // m][middle % m]
            if middle_value == target:
                return True
            elif middle_value < target:
                left = middle + 1
            else:
                right = middle - 1
        return False

class SearchMatrixTest(TestCase):
    def test_1(self):
        self.assertTrue(Solution().searchMatrix([[1,3,5,7],[10,11,16,20],[23,30,34,60]], 3))

    def test_2(self):
        self.assertFalse(Solution().searchMatrix([[1,3,5,7],[10,11,16,20],[23,30,34,60]], 13))

    def test_3(self):
        self.assertTrue(Solution().searchMatrix([[1]], 1))
