from typing import List
from unittest import TestCase

class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        def rec(v: int, adj_list: List[List[int]], in_stack: List[bool], visited: List[bool]):
            visited[v] = True
            in_stack[v] = True
            for u in adj_list[v]:
                if in_stack[u]:
                    return False
                if not visited[u]:
                    if not rec(u, adj_list, in_stack, visited):
                        return False
            in_stack[v] = False
            return True

        n = numCourses
        adj_list: List[List[int]] = [[] for i in range(n)]
        for v, u in prerequisites:
            adj_list[u].append(v)
        visited, in_stack = [False] * n, [False] * n
        for v in range(n):
            if not visited[v]:
                if not rec(v, adj_list, in_stack, visited):
                    return False
        return True

class CanFinishTest(TestCase):    
    def test_1(self):
        self.assertTrue(Solution().canFinish(2, [[1,0]]))

    def test_2(self):
        self.assertFalse(Solution().canFinish(2, [[1,0],[0,1]]))
