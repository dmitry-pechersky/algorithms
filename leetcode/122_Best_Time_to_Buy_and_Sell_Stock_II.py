from typing import List
from unittest import TestCase

class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        profit = 0
        for i in range(1, len(prices)):
            if prices[i] > prices[i - 1]:
                profit += prices[i] - prices[i - 1]
        return profit

class MaxProfitTest(TestCase):
    def test_1(self) -> None:
        self.assertEqual(Solution().maxProfit([7,1,5,3,6,4]), 7)

    def test_2(self) -> None:
        self.assertEqual(Solution().maxProfit([1,2,3,4,5]), 4)

    def test_3(self) -> None:
        self.assertEqual(Solution().maxProfit([7,6,4,3,1]), 0)
