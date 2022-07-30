from typing import List
from unittest import TestCase

class Solution:
    def maximumBags(self, capacity: List[int], rocks: List[int], additionalRocks: int) -> int:
        n, cnt = len(capacity), additionalRocks
        bags = [capacity[i] - rocks[i] for i in range(n)]
        bags.sort()
        for i in range(n):
            if bags[i] > cnt:
                return i
            cnt -= bags[i]
        return n

class MaximumBagsTest(TestCase):
    def test_1(self):
        self.assertEqual(3, Solution().maximumBags([2,3,4,5], [1,2,4,4], 2))

    def test_2(self):
        self.assertEqual(3, Solution().maximumBags([10,2,2], [2,2,0], 100))



