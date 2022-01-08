import unittest
from typing import List

class Solution:
    def rob(self, nums: List[int]) -> int:
        dp1, dp2 = 0, 0
        for num in nums:
            dp1, dp2 = dp2 + num, dp1 if dp1 > dp2 else dp2
        return max(dp1, dp2)

class RobTest(unittest.TestCase):
    def test_1(self) -> None:
        self.assertEqual(Solution().rob([1,2,3,1]), 4) 

    def test_2(self) -> None:
        self.assertEqual(Solution().rob([2,7,9,3,1]), 12)
