import unittest
from typing import List

class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        last_value = nums[0]
        cnt, new_i = 1, 1
        for i in range(1, len(nums)):
            nums[new_i] = nums[i]
            if nums[i] == last_value:
                if cnt < 2:
                    cnt += 1
                    new_i += 1
            else:
                last_value = nums[i]
                cnt = 1
                new_i += 1
        return new_i

class TestRemoveDuplicates(unittest.TestCase):
    def test_1(self):
        nums = [1,1,1,2,2,3]
        self.assertEqual(Solution().removeDuplicates(nums), 5)
        self.assertEqual(nums[:5], [1,1,2,2,3])
            
    def test_2(self):
        nums = [0,0,1,1,1,1,2,3,3]
        self.assertEqual(Solution().removeDuplicates(nums), 7)
        self.assertEqual(nums[:7], [0,0,1,1,2,3,3])
