import unittest
from typing import List

class Solution:
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        merged_intervals = []
        intervals.sort()
        current_start, current_end = intervals[0]
        for start, end in intervals:
            if start <= current_end:
                if end > current_end:
                    current_end = end
            else:
                merged_intervals.append([current_start, current_end])
                current_start, current_end = start, end
        merged_intervals.append([current_start, current_end])
        return merged_intervals        

class MergeTest(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().merge([[1,3],[2,6],[8,10],[15,18]]), [[1,6],[8,10],[15,18]])

    def test_2(self):
        self.assertEqual(Solution().merge([[1,4],[4,5]]), [[1,5]])
