from typing import List
from unittest import TestCase

class Solution:
    def numPairsDivisibleBy60(self, time: List[int]) -> int:
        cnts = [0] * 60
        for t in time:
            cnts[t % 60] += 1
        cnt = cnts[30] * (cnts[30] - 1) // 2 + cnts[0] * (cnts[0] - 1) // 2
        for i in range(1, 30):
            cnt += cnts[i] * cnts[60 - i]
        return cnt

class NumPairsDivisibleBy60Test(TestCase):
    def test_1(self) -> None:
        self.assertEqual(Solution().numPairsDivisibleBy60([30,20,150,100,40]), 3)

    def test_2(self) -> None:
        self.assertEqual(Solution().numPairsDivisibleBy60([60,60,60]), 3)

    def test_3(self) -> None:
        self.assertEqual(Solution().numPairsDivisibleBy60([15,63,451,213,37,209,343,319]), 1)

