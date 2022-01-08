from typing import List
from unittest import TestCase

class Solution:
    def cherryPickup(self, grid: List[List[int]]) -> int:
        n, m = len(grid), len(grid[0])
        dp = [[-1] * m for i in range(m)]
        dp[0][-1] = grid[0][0] + grid[0][-1]
        for row in grid[1:]:
            dp_cur = [[-1] * m for i in range(m)]
            for i in range(m):
                for j in range(m):
                    if dp[i][j] != -1:
                        for ni, nj in ((ni, nj) for ni in (i - 1, i, i + 1) for nj in (j - 1, j, j + 1) if ni >= 0 and ni < m and nj >= 0 and nj < m):
                            v = dp[i][j] + row[ni] + (row[nj] if ni != nj else 0)
                            if dp_cur[ni][nj] < v:
                                dp_cur[ni][nj] = v
            dp = dp_cur
        return max(map(max, dp))

class CherryPickupTest(TestCase):
    def test_1(self) -> None:
        self.assertEqual(Solution().cherryPickup([[3,1,1],[2,5,1],[1,5,5],[2,1,1]]), 24)

    def test_2(self) -> None:
        self.assertEqual(Solution().cherryPickup([[1,0,0,0,0,0,1],[2,0,0,0,0,3,0],[2,0,9,0,0,0,0],[0,3,0,5,4,0,0],[1,0,2,3,0,0,6]]), 28)
