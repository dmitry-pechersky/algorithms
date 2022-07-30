from typing import List, Optional
from unittest import TestCase
from collections import deque

class Solution:
    def secondMinimum(self, n: int, edges: List[List[int]], time: int, change: int) -> int:
        adj_list: List[List[int]] = [[] for i in range(n)]
        for v, u in edges:
            v, u = v -1, u - 1
            adj_list[v].append(u)
            adj_list[u].append(v)
        queue = deque([(0, 0)])
        costs: List[List[Optional[int]]] = [[None, None] for i in range(n)]
        costs[0][0] = 0
        while queue:
            cost, v = queue.popleft()
            if v == n - 1 and costs[v][1] is not None:
                return costs[v][1]
            wait_time = 0 if cost % (2 * change) < change else 2 * change - cost % (2 * change)
            next_cost = cost + time + wait_time
            for u in adj_list[v]:
                if costs[u][0] is None:
                    costs[u][0] = next_cost
                elif costs[u][0] < next_cost and costs[u][1] is None:
                    costs[u][1] = next_cost
                else:
                    continue
                queue.append((next_cost, u))
        return  -1


class TestSecondMinimum(TestCase):
    def test_1(self):
        self.assertEqual(13, Solution().secondMinimum(5, [[1,2],[1,3],[1,4],[3,4],[4,5]], 3, 5))
    
    def test_2(self):
        self.assertEqual(11, Solution().secondMinimum(2, [[1,2]], 3, 2))        
