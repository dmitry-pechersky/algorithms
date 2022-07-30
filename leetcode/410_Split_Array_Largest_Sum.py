from typing import List, Dict, Tuple, Optional
from unittest import TestCase

class Solution:
    def splitArray(self, nums: List[int], m: int) -> int:
        n = len(nums)
        dp = [[0] * n for i in range(m + 1)]
        dp[1][0] = nums[0]
        for i in range(1, n):
            dp[1][i] = dp[1][i - 1] + nums[i]
            for mi in range(2, min(m, i + 1) + 1):
                dp[mi][i] = nums[i] if nums[i] > dp[mi - 1][i - 1] else dp[mi - 1][i - 1]
                s_sum = nums[i]
                for k in range(i - 2, mi - 3, -1):
                    s_sum += nums[k + 1]
                    if s_sum >= dp[mi][i]:
                        break 
                    s_max = s_sum if s_sum > dp[mi - 1][k] else dp[mi - 1][k]
                    if s_max < dp[mi][i]:
                        dp[mi][i] = s_max
        return dp[m][n - 1]


class Solution2:
    def splitArray(self, nums: List[int], m: int) -> int:
        def rec(s: int, m: int, dp: List[List[Optional[int]]]) -> int:
            if m == 1:
                return sums[-1] - sums[s]
            if dp[m][s] is None:
                min_largest_sum = 10**9
                for i in range(s + 1, n - m + 2):
                    cur_sum = sums[i] - sums[s]
                    if cur_sum > min_largest_sum:
                        break
                    rec_sum = rec(i, m - 1, dp)
                    if cur_sum < rec_sum:
                        cur_sum = rec_sum
                    if cur_sum < min_largest_sum:
                        min_largest_sum = cur_sum
                dp[m][s] = min_largest_sum
            return dp[m][s]            

        n = len(nums)
        sums = [0] * (n + 1)
        for i in range(0, n):
            sums[i + 1] += sums[i] + nums[i]
        dp: List[List[Optional[int]]] = [[None] * n for i in range(m + 1)]
        return rec(0, m, dp)

if __name__ == "__main__":
    Solution().splitArray([0,8,1,4], 4)

class SplitArrayTest(TestCase):
    
    def test_1(self) -> None:
        self.assertEqual(18, Solution().splitArray([7,2,5,10,8], 2))
    
    def test_2(self) -> None:
        self.assertEqual(9, Solution().splitArray([1,2,3,4,5], 2))
    
    def test_3(self) -> None:
        self.assertEqual(4, Solution().splitArray([1,4,4], 3))
        
    def test_5(self) -> None:
        self.assertEqual(8, Solution().splitArray([0,8,1,4], 4))
    
    def test_6(self) -> None:
        self.assertEqual(4, Solution().splitArray([2,3,1,2,4,3], 5))
    
    def test_4(self) -> None:
        self.assertEqual(10, Solution().splitArray([10,2,3], 2))
