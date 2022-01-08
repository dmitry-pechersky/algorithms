import unittest
from typing import List

class Solution:
    def threeSumClosest(self, nums: List[int], target: int) -> int:
        n = len(nums)
        nums.sort()
        closest = nums[0] + nums[1] + nums[2]
        for i in range(n - 2):
            j, k = i + 1, n - 1
            while j < k:
                s = nums[i] + nums[j] + nums[k]
                if s == target:
                    return s
                elif s > target:
                    k -= 1
                else:
                    j += 1
                if abs(target - s) < abs(target - closest):
                    closest = s
        return closest

class Test3SumClosest(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().threeSumClosest([-1,2,1,-4], 1), 2)    

    def test_2(self):
        self.assertEqual(Solution().threeSumClosest([0,0,0], 1), 0)    

