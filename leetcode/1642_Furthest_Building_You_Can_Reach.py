from typing import List
from unittest import TestCase
from heapq import heappop, heappush


class Solution:
    def furthestBuilding(self, heights: List[int], bricks: int, ladders: int) -> int:
        prev_height = heights[0]
        heap: List[int] = []
        for i in range(1, len(heights)):
            height = heights[i]
            if height > prev_height:
                heappush(heap, - (height - prev_height))
                bricks -= height - prev_height
                while bricks < 0:
                    if ladders == 0 or len(heap) == 0:
                        return i - 1
                    moving_height = - heappop(heap)
                    bricks += moving_height
                    ladders -= 1
            prev_height = height
        return len(heights) - 1



class FurthestBuildingTest(TestCase):
    def test_1(self):
        self.assertEqual(4, Solution().furthestBuilding([4,2,7,6,9,14,12], 5, 1))

    def test_2(self):
        self.assertEqual(7, Solution().furthestBuilding([4,12,2,7,3,18,20,3,19], 10, 2))

    def test_3(self):
        self.assertEqual(3, Solution().furthestBuilding([14,3,19,3], 17, 0))        

        