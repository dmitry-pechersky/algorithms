from unittest import TestCase
from collections import deque
from typing import Set

class Solution:
    def minDays(self, n: int) -> int:
        queue = deque([(0, n)])
        cache: Set[int] = set()
        while queue:
            days, oranges = queue.popleft()
            if oranges == 0:
                return days
            if oranges % 3 == 0:
                next_oranges = oranges - 2 * int(oranges / 3)
                if next_oranges not in cache:
                    queue.append((days + 1, next_oranges))
                    cache.add(next_oranges)
            if oranges % 2 == 0:
                next_oranges = int(oranges / 2)
                if next_oranges not in cache:
                    queue.append((days + 1, next_oranges))
                    cache.add(next_oranges)
            if oranges - 1 not in cache:
                next_oranges = oranges - 1
                queue.append((days + 1, next_oranges))
                cache.add(next_oranges)
        raise ValueError("Unreachable")

class MinDaysTest(TestCase):
    def test_1(self):
        self.assertEqual(4, Solution().minDays(10))
    
    def test_2(self):
        self.assertEqual(3, Solution().minDays(6))
    