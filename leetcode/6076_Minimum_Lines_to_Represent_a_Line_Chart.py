from typing import List
from unittest import TestCase

class Solution:
    def minimumLines(self, stockPrices: List[List[int]]) -> int:
        n = len(stockPrices)
        if n <= 1:
            return 0
        stocks = sorted(stockPrices, key=lambda pair: pair[0])
        cnt, days, slope = 1, stocks[1][0] - stocks[0][0], stocks[1][1] - stocks[0][1]
        for i in range(2, n):
            i_days = stocks[i][0] - stocks[i - 1][0]
            i_slope = stocks[i][1] - stocks[i - 1][1]
            if days * i_slope != i_days * slope:
                cnt += 1
                days, slope = i_days, i_slope
        return cnt
        

class MinimumLinesTest(TestCase):
    def test_1(self):
        self.assertEqual(3, Solution().minimumLines([[1,7],[2,6],[3,5],[4,4],[5,4],[6,3],[7,2],[8,1]]))

    def test_2(self):
        self.assertEqual(1, Solution().minimumLines([[3,4],[1,2],[7,8],[2,3]]))
