from typing import List
from unittest import TestCase
from collections import deque

class Solution:
    def minJumps(self, arr: List[int]) -> int:
        n = len(arr)
        dic = {}
        for i, val in enumerate(arr):
            if val not in dic:
                dic[val] = []
            dic[val].append(i)
        queue = deque([(0,0)])
        appended = [False] * n
        appended[0] = True
        while queue:
            cost, i = queue.popleft()
            if i == n - 1:
                return cost
            if i > 0 and not appended[i - 1]:
                appended[i - 1] = True
                queue.append((cost + 1, i - 1))
            if not appended[i + 1]:
                appended[i + 1] = True
                queue.append((cost + 1, i + 1))
            if arr[i] in dic:
                for next_i in dic[arr[i]]:
                    if not appended[next_i]:
                        appended[next_i] = True
                        queue.append((cost + 1, next_i))
                dic.pop(arr[i])

class MinJumpsTes(TestCase):
    def test_1(self) -> None:
        self.assertEqual(Solution().minJumps([100,-23,-23,404,100,23,23,23,3,404]), 3)

    def test_2(self) -> None:
        self.assertEqual(Solution().minJumps([7]), 0)

    def test_3(self) -> None:
        self.assertEqual(Solution().minJumps([7,6,9,6,9,6,9,7]), 1)        