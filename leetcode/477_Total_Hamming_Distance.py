import unittest
from typing import List

class Solution:
    def totalHammingDistance(self, nums: List[int]) -> int:
        n, cnt = len(nums), 0
        for i in range(32):
            one_bit = 1 << i
            one_bint_cnt = 0
            for num in nums:
                one_bint_cnt += (num & one_bit) >> i
            cnt += one_bint_cnt * (n -one_bint_cnt)
        return cnt 

class TotalHammingDistanceTest(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().totalHammingDistance([4,14,2]), 6)    

    def test_2(self):
        self.assertEqual(Solution().totalHammingDistance([4,14,4]), 4)    
