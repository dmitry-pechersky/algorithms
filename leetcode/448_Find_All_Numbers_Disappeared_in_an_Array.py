class Solution:
    def findDisappearedNumbers(self, nums: List[int]) -> List[int]:
        cnts = [False] * len(nums)
        for num in nums:
            cnts[num - 1] = True
        return [i for i in range(1, len(nums) + 1) if not cnts[i -1]]
        