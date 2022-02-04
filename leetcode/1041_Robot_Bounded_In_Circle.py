from unittest import TestCase
from typing import Tuple

class Solution:
    def isRobotBounded(self, instructions: str) -> bool:
        def next(state: Tuple[int, int, int, int], action: str) -> Tuple[int, int, int, int]:
            x, y, dx, dy = state
            new_state = None
            if action == "G":
                new_state = (x + dx, y + dy, dx, dy)
            elif action == 'R':
                new_state = (x, y, -dy, dx)
            else:
                new_state = (x, y, dy, -dx)
            return new_state
        cur_state = (0, 0, 1, 0)
        for action in instructions:
            cur_state = next(cur_state, action)
        x, y, dx, dy = cur_state
        return (x == 0 and y == 0) or not (dx == 1 and dy == 0)


class IsRobotBoundedTest(TestCase):
    def test_1(self) -> None:
        self.assertTrue(Solution().isRobotBounded("GGLLGG"))

    def test_2(self) -> None:
        self.assertFalse(Solution().isRobotBounded("GG"))

    def test_3(self) -> None:
        self.assertTrue(Solution().isRobotBounded("GL"))
