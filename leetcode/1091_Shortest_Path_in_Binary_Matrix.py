from typing import List
from unittest import TestCase
from collections import deque

class Solution:
    def shortestPathBinaryMatrix(self, grid: List[List[int]]) -> int:
        n = len(grid)
        dxdy = [(i, j) for i in (-1, 0, 1) for j in (-1, 0, 1) if i != 0 or j != 0]        
        if grid[0][0] == 0:
            queue = deque([(1, 0, 0)])
            while queue:
                cost, x, y = queue.popleft()
                if x == n - 1 and y == n - 1:
                    return cost
                for x1, y1 in ((x + dx, y + dy) for dx, dy in dxdy):
                    if 0 <= x1 < n and 0 <= y1 < n and grid[x1][y1] == 0:
                        queue.append((cost + 1, x1, y1))
                        grid[x1][y1] = 1
        return -1

class ShortestPathBinaryMatrixTest(TestCase):
    
    def test_1(self):
        self.assertEqual(2, Solution().shortestPathBinaryMatrix([[0,1],[1,0]]))
            
    def test_2(self):
        self.assertEqual(4, Solution().shortestPathBinaryMatrix([[0,0,0],[1,1,0],[1,1,0]]))

    def test_3(self):
        self.assertEqual(-1, Solution().shortestPathBinaryMatrix([[1,0,0],[1,1,0],[1,1,0]]))
    