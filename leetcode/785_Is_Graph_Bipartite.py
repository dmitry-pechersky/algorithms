from typing import List
from unittest import TestCase

class Solution:
    def isBipartite(self, graph: List[List[int]]) -> bool:
        n = len(graph)
        colours = [None] * n
        for i in range(n):
            if colours[i] is None:
                stack, colours[i] = [i], 0
                while stack:
                    v = stack.pop()
                    for u in graph[v]:
                        if colours[u] is None:
                            colours[u] = not colours[v]
                            stack.append(u)
                        elif colours[v] == colours[u]:
                            return False
        return True

class IsBipartiteTest(TestCase):
    def test_1(self) -> None:
        self.assertFalse(Solution().isBipartite([[1,2,3],[0,2],[0,1,3],[0,2]]))

    def test_2(self) -> None:
        self.assertTrue(Solution().isBipartite([[1,3],[0,2],[1,3],[0,2]]))



