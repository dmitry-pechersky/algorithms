import unittest
from typing import List

class Solution:
    def jump(self, nums: List[int]) -> int:
        n = len(nums)
        jump_max, next_jump_max, jump_cnt = 0, 0 , 0
        for i in range(n - 1):
            if i > jump_max:
                jump_max = next_jump_max
                jump_cnt += 1
            if next_jump_max < i + nums[i]:
                next_jump_max = i + nums[i]
            if next_jump_max >= n - 1:
                jump_cnt += 1
                break
        return jump_cnt        

class TestJump(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().jump([2,3,1,1,4]), 2)

    def test_2(self):
        self.assertEqual(Solution().jump([2,3,0,1,4]), 2)

    def test_3(self):
        self.assertEqual(Solution().jump([2,1]), 1)

    def test_4(self):
        self.assertEqual(Solution().jump([0]), 0)
