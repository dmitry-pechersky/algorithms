from typing import List
from unittest import TestCase

class Solution:
    def uniquePathsIII(self, grid: List[List[int]]) -> int:
        n, m = len(grid), len(grid[0])
        start, empty_n= (0, 0), 0
        in_stack = [[False] * m for _ in  range(n)]
        path_cnt, in_stack_cnt = 0, 0
        for x in range(n):
            for y in range(m):
                if grid[x][y] == 0:
                    empty_n += 1
                elif grid[x][y] == 1:
                    start = (x, y)
        
        def dfs(x: int, y: int):
            nonlocal in_stack_cnt
            nonlocal in_stack
            nonlocal path_cnt
            in_stack[x][y] = True
            in_stack_cnt += 1
            for dx, dy in ((0, 1), (1, 0), (0, -1), (-1, 0)):
                x1, y1 = x + dx, y + dy
                if 0 <= x1 < n and 0 <= y1 < m:
                    if grid[x1][y1] == 0 and not in_stack[x1][y1]:
                        dfs(x1, y1)
                    elif grid[x1][y1] == 2 and in_stack_cnt == empty_n + 1:
                        path_cnt += 1
            in_stack[x][y] = False
            in_stack_cnt -= 1

        dfs(*start)
        return path_cnt

class UniquePathsIIITest(TestCase):
    def test_1(self):
        self.assertEqual(2, Solution().uniquePathsIII([[1,0,0,0],[0,0,0,0],[0,0,2,-1]]))

    def test_2(self):
        self.assertEqual(4, Solution().uniquePathsIII([[1,0,0,0],[0,0,0,0],[0,0,0,2]]))

    def test_3(self):
        self.assertEqual(0, Solution().uniquePathsIII([[0,1],[2,0]]))

