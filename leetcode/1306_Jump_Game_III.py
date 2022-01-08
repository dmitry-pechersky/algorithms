from typing import List
import unittest

class Solution:
    def canReach(self, arr: List[int], start: int) -> bool:
        n = len(arr)
        visited, stack = [False] * n, [start]
        while stack:
            node = stack.pop()
            for successor in (node - arr[node], node + arr[node]):
                if successor >= 0 and successor < n:
                    if arr[successor] == 0:
                        return True
                    if not visited[successor]:
                        visited[successor] = True
                        stack.append(successor)
        return False
        
class CanReachTest(unittest.TestCase):
    def test_1(self) -> None:
        self.assertTrue(Solution().canReach([4,2,3,0,3,1,2], 5))

    def test_2(self) -> None:
        self.assertTrue(Solution().canReach([4,2,3,0,3,1,2], 0))

    def test_3(self) -> None:
        self.assertFalse(Solution().canReach([3,0,2,1,2], 2))
