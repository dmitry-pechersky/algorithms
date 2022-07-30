from typing import List
from unittest import TestCase

class Solution:
    def criticalConnections(self, n: int, connections: List[List[int]]) -> List[List[int]]:
        adj_list: List[List[int]] = [[] for i in range(n)]
        for v, u in connections:
            adj_list[v].append(u)
            adj_list[u].append(v)
        depths = [0] * n
        res: List[List[int]] = []

        def rec(v: int, depth: int, w):
            nonlocal adj_list
            nonlocal depths
            nonlocal res
            depths[v] = depth
            for u in adj_list[v]:
                if u != w:
                    if depths[u] == 0:
                        rec(u, depth + 1, v)
                        if depths[u] == depth + 1:
                            res.append([u, v])
                    if depths[u] < depths[v]:
                        depths[v] = depths[u]

        rec(0, 1, -1)
        return res

class CriticalConnections(TestCase):
    def test_1(self):
        self.assertEqual(
            [[1,3]], 
            list(map(lambda edge: sorted(edge), Solution().criticalConnections(4, [[0,1],[1,2],[2,0],[1,3]])))
            )


    def test_2(self):
        self.assertEqual(
            [[0,1]], 
            list(map(lambda edge: sorted(edge), Solution().criticalConnections(2, [[0,1]])))
            )


