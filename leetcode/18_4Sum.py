import unittest
from typing import List

class Solution:
    def fourSum(self, nums: List[int], target: int) -> List[List[int]]:
        nums.sort()
        quadruplets = []
        n = len(nums)
        prev_num_i = None
        for i in range(0, n - 3):
            if nums[i] != prev_num_i:
                prev_num_i = nums[i]
                prev_num_j = None
                for j in range(i + 1, n - 2):
                    if nums[j] != prev_num_j:
                        prev_num_j = nums[j]
                        k, l = j + 1, n - 1
                        prev_num_k, prev_num_l = None, None
                        while k < l:
                            nums_sum = nums[i] + nums[j] + nums[k] + nums[l] 
                            if nums_sum == target:
                                if nums[k] != prev_num_k or nums[l] != prev_num_l:
                                    quadruplets.append([nums[i], nums[j], nums[k], nums[l]])
                                    prev_num_k, prev_num_l = nums[k], nums[l]
                                k += 1
                                l -= 1
                            elif nums_sum > target:
                                l -= 1
                            else:
                                k += 1
        return quadruplets

class Test4Sum(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().fourSum([1, 0, -1, 0, -2, 2], 0), [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]])    

    def test_2(self):
        self.assertEqual(Solution().fourSum([2, 2, 2, 2, 2], 8), [[2, 2, 2, 2]])    

    def test_3(self):
        self.assertEqual(Solution().fourSum([2, 2, 2, 2, 2, 2, 2, 2, 2], 8), [[2, 2, 2, 2]])    
