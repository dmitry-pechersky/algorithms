from unittest import TestCase
from typing import List
from heapq import heapify, heappop, heappush

class Solution:
    def minimumObstacles(self, grid: List[List[int]]) -> int:
        n, m = len(grid), len(grid[0])
        queue = [(0, 0, 0)]
        heapify(queue)
        visited = [[None] * m for _ in range(n)]
        visited[0][0] == 0
        while queue:
            cost, x, y = heappop(queue)
            if x == n - 1 and y == m - 1:
                return cost
            for dx, dy in ((1, 0), (0, 1), (-1, 0), (0, -1)):
                x1, y1 = x + dx, y + dy
                if 0 <= x1 < n and 0 <= y1 < m:
                    cost1 = cost if grid[x1][y1] == 0 else cost + 1
                    if visited[x1][y1] is None or visited[x1][y1] > cost1:
                        visited[x1][y1] = cost1
                        heappush(queue, (cost1, x1, y1))
        return -1

class MinimumObstaclesTest(TestCase):
    def test_1(self):
        self.assertEqual(Solution().minimumObstacles([[0,1,1],[1,1,0],[1,1,0]]), 2)

    def test_2(self):
        self.assertEqual(Solution().minimumObstacles([[0,1,0,0,0],[0,1,0,1,0],[0,0,0,1,0]]), 0)
