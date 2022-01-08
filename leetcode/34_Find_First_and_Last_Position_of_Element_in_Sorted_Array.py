import unittest
from typing import List

class Solution:
    def _binary_search_left(self, nums: List[int], target: int, left: int, right: int):
        while left < right:
            mid = (left + right) // 2
            if nums[mid] > target:
                right = mid - 1
            elif nums[mid] < target:
                left = mid + 1
            else:
                right = mid
        return left if nums[left] == target else -1
        
    def _binary_search_right(self, nums: List[int], target: int, left: int, right: int):
        while left < right:
            mid = (left + right + 1) // 2
            if nums[mid] > target:
                right = mid - 1
            elif nums[mid] < target:
                left = mid + 1
            else:
                left = mid
        return left if nums[left] == target else -1

    def searchRange(self, nums: List[int], target: int) -> List[int]:
        n, left, right = len(nums), -1, -1
        if n != 0:
            left = self._binary_search_left(nums, target, 0, n - 1)
            if left != -1:
                right = self._binary_search_right(nums, target, left, n - 1)
        return [left, right]

class TestSearchRange(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().searchRange([5,7,7,8,8,10], 8), [3,4])    

    def test_2(self):
        self.assertEqual(Solution().searchRange([5,7,7,8,8,10], 6), [-1, -1])    

    def test_3(self):
        self.assertEqual(Solution().searchRange([], 0), [-1, -1])    

class TestBinarySearchLeft(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution()._binary_search_left([2,5,5,5,5,8], 5, 0, 5), 1)       

    def test_2(self):
        self.assertEqual(Solution()._binary_search_left([5,5,5,5,5,8], 5, 0, 5), 0)       

    def test_3(self):
        self.assertEqual(Solution()._binary_search_left([5,5,5,5,5,8], 6, 0, 5), -1)       

class TestBinarySearchRight(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution()._binary_search_right([2,5,5,5,5,8], 5, 0, 5), 4)       

    def test_2(self):
        self.assertEqual(Solution()._binary_search_right([4, 5,5,5,5,5], 5, 0, 5), 5)       

    def test_3(self):
        self.assertEqual(Solution()._binary_search_right([5,5,5,5,5,8], 6, 0, 5), -1)       
