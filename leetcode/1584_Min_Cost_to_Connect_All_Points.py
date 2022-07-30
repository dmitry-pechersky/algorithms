from typing import List
from unittest import TestCase

class DisjointSet: 
    def __init__(self, n: int) -> None:
        self.parents = [i for i in range(n)]
        self.ranks = [0] * n

    def root(self, i: int) -> int:
        while i != self.parents[i]:
            i, self.parents[i] = self.parents[i], self.parents[self.parents[i]]
        return i

    def join(self, i: int, j: int) -> None:
        root_i, root_j = (self.root(i), self.root(j))
        if self.ranks[root_i] < self.ranks[root_j]:
            self.parents[root_i] = root_j
        elif self.ranks[root_i] > self.ranks[root_j]:
            self.parents[root_j] = root_i
        else:
            self.parents[root_i] = root_j
            self.ranks[root_j] += 1

class Solution(object):
    def minCostConnectPoints(self, points: List[List[int]]) -> int:
        n = len(points)
        ds, edges = DisjointSet(n), []
        total_cost, edge_cnt = 0, 0

        for i in range(n):
            for j in range(i + 1, n):
                edges.append((abs(points[i][0] - points[j][0]) + abs(points[i][1] - points[j][1]), i ,j))
        edges.sort()
        for cost, i, j in edges:
            if ds.root(i) != ds.root(j):
                ds.join(i, j)
                total_cost += cost
                edge_cnt += 1
                if edge_cnt == n - 1:
                    break
        return total_cost

class MinCostConnectPointsTest(TestCase):
    def test_1(self) -> None:
        self.assertEqual(20, Solution().minCostConnectPoints([[0,0],[2,2],[3,10],[5,2],[7,0]]))

    def test_2(self) -> None:
        self.assertEqual(18, Solution().minCostConnectPoints([[3,12],[-2,5],[-4,1]]))