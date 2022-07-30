from unittest import TestCase
from heapq import heapify, heappop, heappush
from typing import List

class Solution:
    def minimumEffortPath(self, heights: List[List[int]]) -> int:
        rows, columns = len(heights), len(heights[0])
        expanded = [[None] * columns for _ in range(rows)]
        expanded[0][0] = 0
        heap = [(0, (0, 0))]
        while heap:
            cost, (x, y)  = heappop(heap)
            if (x, y) == (rows - 1, columns - 1):
                return cost
            if cost <= expanded[x][y]:
                for x1, y1 in ((x + dx, y + dy) for dx, dy in ((1, 0), (0, 1), (-1, 0), (0, -1)) if  0 <= x + dx < rows and 0 <= y + dy < columns):
                    move_cost = abs(heights[x][y] - heights[x1][y1])
                    cost1 = move_cost if move_cost > cost else cost
                    if expanded[x1][y1] is None or cost1 < expanded[x1][y1]:
                        expanded[x1][y1] = cost1
                        heappush(heap, (move_cost if move_cost > cost else cost, (x1, y1,)))

class Solution:
    def minimumEffortPath(self, heights: List[List[int]]) -> int:
        rows, columns = len(heights), len(heights[0])
        expanded = [[None] * columns for _ in range(rows)]
        expanded[0][0] = 0
        heap = [(0, (0, 0))]
        while heap:
            cost, (x, y)  = heappop(heap)
            if (x, y) == (rows - 1, columns - 1):
                return cost
            if cost <= expanded[x][y]:
                for x1, y1 in ((x + dx, y + dy) for dx, dy in ((1, 0), (0, 1), (-1, 0), (0, -1)) if  0 <= x + dx < rows and 0 <= y + dy < columns):
                    move_cost = abs(heights[x][y] - heights[x1][y1])
                    cost1 = move_cost if move_cost > cost else cost
                    if expanded[x1][y1] is None or cost1 < expanded[x1][y1]:
                        expanded[x1][y1] = cost1
                        heappush(heap, (move_cost if move_cost > cost else cost, (x1, y1,)))

                        
class MinimumEffortPathTest(TestCase):
    def test1_(self) -> None:
        self.assertEqual(2, Solution().minimumEffortPath([[1,2,2],[3,8,2],[5,3,5]]))

    def test2_(self) -> None:
        self.assertEqual(1, Solution().minimumEffortPath([[1,2,3],[3,8,4],[5,3,5]]))

    def test3_(self) -> None:
        self.assertEqual(0, Solution().minimumEffortPath([[1,2,1,1,1],[1,2,1,2,1],[1,2,1,2,1],[1,2,1,2,1],[1,1,1,2,1]]))