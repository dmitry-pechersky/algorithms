import unittest
from typing import List

class Solution:
    def _binary_search(self, nums: List[int], target: int, left: int , right: int) -> int:
        while left <= right:
            mid = (left + right) // 2
            if nums[mid] > target:
                right = mid - 1
            elif nums[mid] < target:
                left = mid + 1
            else:
                return mid
        return -1 

    def search(self, nums: List[int], target: int) -> int:
        n = len(nums)
        left, right = 0, n - 1
        while left + 1 < right:
            mid = (left + right) // 2 
            if nums[mid] > nums[right]:
                left = mid
            else: 
                right = mid
        smallest = left if nums[left] < nums[right] else right

        if smallest == 0:
            left, right = 0, n - 1 
        elif target >= nums[0]:
            left, right = 0, smallest - 1
        else:
            left, right = smallest, n - 1
        
        return self._binary_search(nums, target, left, right)

class TestSearch(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().search([4,5,6,7,8,9,10,11,12,13,14,0,1,2], 0), 11)    

    def test_2(self):
        self.assertEqual(Solution().search([4,5,6,7,0,1,2], 3), -1)    

    def test_3(self):
        self.assertEqual(Solution().search([1], 0), -1)    

    def test_4(self):
        self.assertEqual(Solution().search([1,3], 3), 1)    
