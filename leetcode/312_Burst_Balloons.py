from typing import List
import unittest

class Solution:
    def maxCoins(self, nums: List[int]) -> int:
        n = len(nums)
        nums = [1] + nums + [1]
        dp = [[0] * (n + 1) for i in range(n + 1)]
        for win in range(1, n + 1):
            for left in range(1, n - win + 2):
                right = left + win - 1
                for i in range(left, right + 1):
                    v =  (dp[left][i - 1] if i > left else 0) + (dp[i + 1][right] if i < right else 0) + nums[left - 1] * nums[i] * nums[right + 1]
                    if v > dp[left][right]: dp[left][right] = v
        return dp[1][n]

class MaxCoinsTest(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().maxCoins([3,1,5,8]), 167)    

    def test_2(self):
        self.assertEqual(Solution().maxCoins([1,5]), 10)    
