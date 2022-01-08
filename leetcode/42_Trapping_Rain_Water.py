import unittest
from typing import List

class Solution:
    def trap(self, height: List[int]) -> int:
        n = len(height)
        max_height, s = 0, 0
        for i in range(n):
            if height[i] < max_height:
                s += max_height - height[i]
            else:
                max_height = height[i]
        i = n - 1
        max_right_height = 0
        while height[i] < max_height:
            if height[i] > max_right_height:
                max_right_height = height[i]
            s -= max_height - max_right_height
            i -= 1
        return s

class TestTrap(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().trap([0,1,0,2,1,0,1,3,2,1,2,1]), 6)

    def test_2(self):
        self.assertEqual(Solution().trap([4,2,0,3,2,5]), 9)
