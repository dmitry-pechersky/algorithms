from typing import List, Tuple
from unittest import TestCase

class Solution:
    def maxChunksToSorted(self, arr: List[int]) -> int:
        i = 0
        for j in range(1, len(arr)):
            m = arr[j]
            while i >= 0 and arr[i] > arr[j]:
                m = m if m > arr[i] else arr[i]
                i -= 1
            i += 1
            arr[i] = m
        return i + 1


class Solution_:
    def maxChunksToSorted(self, arr: List[int]) -> int:
        stack: List[Tuple[int, int]] = []
        for num in arr:
            chunk = (num, num)
            while stack and stack[-1][1] > chunk[0]:
                prev_chunk = stack.pop()
                chunk = (
                    prev_chunk[0] if prev_chunk[0] < chunk[0] else chunk[0],
                    prev_chunk[1] if prev_chunk[1] > chunk[1] else chunk[1]
                    )
            stack.append(chunk)
        return len(stack)


class MaxChunksToSortedTest(TestCase):
    def test_3(self):
        self.assertEqual(1, Solution().maxChunksToSorted([1]))
    
    def test_1(self):
        self.assertEqual(1, Solution().maxChunksToSorted([5,4,3,2,1]))

    def test_2(self):
        self.assertEqual(4, Solution().maxChunksToSorted([2,1,3,4,4]))
    