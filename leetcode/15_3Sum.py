import unittest
from typing import List


class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        n = len(nums)
        res = []
        nums.sort()
        i = 0
        while i < n - 2 and nums[i] <= 0:
            if i == 0 or nums[i] > nums[i - 1]:
                j, k = i + 1, n - 1
                while j < k and nums[i] + nums[j] <= 0:
                    if j > i + 1 and nums[j] == nums[j - 1]:
                        j += 1
                    elif k < n - 1 and nums[k] == nums[k + 1]:
                        k -= 1 
                    else:
                        val = nums[i] + nums[j] + nums [k]
                        if val == 0:
                            res.append([nums[i], nums[j], nums[k]])
                            j += 1
                        elif val > 0:
                            k -= 1
                        else:
                            j += 1
            i += 1
        return res

class Test3Sum(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().threeSum([-1,0,1,2,-1,-4]), [[-1,-1,2],[-1,0,1]])    

    def test_2(self):
        self.assertEqual(Solution().threeSum([]), [])    

    def test_3(self):
        self.assertEqual(Solution().threeSum([0]), [])    
