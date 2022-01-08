from typing import List
import unittest

class Solution:
    def largestRectangleArea(self, heights: List[int]) -> int:
        heights.append(0)
        max_area, stack = 0, []
        for i in range(len(heights)):
            j, height_j = i, heights[i]
            while stack:
                k, height_k = stack[-1]
                if height_k > height_j:
                    area = (i - k) *  height_k
                    if area > max_area:
                        max_area = area
                    j = k
                    stack.pop()
                elif height_k == height_j:
                    j = k
                    stack.pop()
                else:
                    break
            if height_j > 0:
                stack.append((j, height_j)) 
        return max_area

class LargestRectangleAreaTes(unittest.TestCase):
    def test_1(self) -> None:
        self.assertEqual(Solution().largestRectangleArea([2,1,5,6,2,3]), 10)

    def test_2(self) -> None:
        self.assertEqual(Solution().largestRectangleArea([2,4]), 4)

    def test_3(self) -> None:
        self.assertEqual(Solution().largestRectangleArea([1, 1]), 2)
        
    def test_4(self) -> None:
        self.assertEqual(Solution().largestRectangleArea([2, 1, 2]), 3)
