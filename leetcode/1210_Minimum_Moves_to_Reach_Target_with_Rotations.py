
from unittest import TestCase
from typing import List, Tuple, Iterator, Set
from collections import deque

class Solution:
    def minimumMoves(self, grid: List[List[int]]) -> int:
        def succcessors(state: Tuple[int, int, bool]) -> Iterator[Tuple[int, int, bool]]:
            x, y, horizontal = state
            if horizontal:
                if x + 1 < n and y + 1 < n and grid[x + 1][y] == 0 and grid[x + 1][y + 1] == 0:
                    yield (x + 1, y, True)
                if x + 1 < n and y + 1 < n and grid[x + 1][y] == 0 and grid[x + 1][y + 1] == 0:
                    yield (x, y, False)
                if y + 2 < n and grid[x][y + 2] == 0:
                    yield (x, y + 1, True)
            else:
                if x + 1 < n and y + 1 < n and grid[x][y + 1] == 0 and grid[x + 1][y + 1] == 0:
                    yield (x, y + 1, False)
                if y + 1 < n and x + 1 < n and grid[x][y + 1] == 0 and grid[x + 1][y + 1] == 0:
                    yield (x, y, True)
                if x + 2 < n and grid[x + 2][y] == 0:
                    yield (x + 1, y, False)

        n = len(grid)
        state = (0, 0, True)
        queue = deque([(0, state)])
        visited: Set[Tuple[int, int, bool]] = { state }
        while queue:
            cost, state  = queue.popleft()
            if state == (n - 1, n - 2, True):
                return cost
            for next_state in succcessors(state):
                if next_state not in visited:
                    visited.add(next_state)
                    queue.append((cost + 1, next_state))
        return -1

class MinimumMovesTest(TestCase):
    def test_1(self) -> None:
        grid = [
            [0,0,0,0,0,1],
            [1,1,0,0,1,0],
            [0,0,0,0,1,1],
            [0,0,1,0,1,0],
            [0,1,1,0,0,0],
            [0,1,1,0,0,0]]
        self.assertEqual(11, Solution().minimumMoves(grid))

    def test_2(self) -> None:
        grid = [[0,0,1,1,1,1],
               [0,0,0,0,1,1],
               [1,1,0,0,0,1],
               [1,1,1,0,0,1],
               [1,1,1,0,0,1],
               [1,1,1,0,0,0]]
        self.assertEqual(9, Solution().minimumMoves(grid))

    def test_3(self) -> None:
        grid = [
            [0,0,1,0,0,0,0,0,0,0,0,0,0,0,0],
            [0,1,0,1,1,0,0,1,0,0,0,0,1,0,0],
            [0,1,0,0,0,0,1,0,0,1,0,0,0,0,0],
            [0,0,0,0,0,0,1,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,1,1,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,1,0,1,0,0,0],
            [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,1,0,1,0,0,1,0,0,0,1,0,0],
            [0,0,0,0,1,0,0,0,0,0,0,0,0,1,0],
            [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,1,0,0,0,0,0,0,0,0,0,0,0],
            [1,0,1,1,0,0,0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0,0,0,0,0,0,1,0],
            [1,0,0,0,0,0,1,0,0,0,1,0,0,0,1],
            [0,0,1,0,1,0,0,0,0,0,0,0,0,0,0]]
        self.assertEqual(-1, Solution().minimumMoves(grid))
