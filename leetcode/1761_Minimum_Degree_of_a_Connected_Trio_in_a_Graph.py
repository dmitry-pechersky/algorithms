from typing import List
from unittest import TestCase

class Solution1:
    def minTrioDegree(self, n: int, edges: List[List[int]]) -> int:
        adj_matrix = [[False] * n for _ in range(n)]
        edges_cnt = [0] * n
        min_edges_cnt = n * 3
        for u, v in edges:
            u, v = u -1, v - 1
            adj_matrix[u][v] = True
            adj_matrix[v][u] = True
            edges_cnt[u] += 1
            edges_cnt[v] += 1
        for v in range(n):
            for u in range(v + 1, n):
                for w in range(u + 1, n):
                    if adj_matrix[v][u] and adj_matrix[v][w] and adj_matrix[u][w]:
                        cnt = edges_cnt[v] + edges_cnt[u] + edges_cnt[w]
                        if cnt < min_edges_cnt:
                            min_edges_cnt = cnt
        return -1 if min_edges_cnt == n * 3 else min_edges_cnt - 6

class Solution2:
    def minTrioDegree(self, n: int, edges: List[List[int]]) -> int:
        adj_matrix = [[False] * n for _ in range(n)]
        edges_cnt = [0] * n
        min_edges_cnt = n * 3

        for v, u in edges:
            v, u = (v - 1, u - 1) if v < u else (u - 1, v - 1)
            adj_matrix[v][u] = True
            edges_cnt[u] += 1
            edges_cnt[v] += 1

        for v,u in edges:
            v, u = (v - 1, u - 1) if v < u else (u - 1, v - 1)
            for w in range(u + 1, n):
                if w != v and w != u and adj_matrix[v][w] and adj_matrix[u][w]:
                    cnt = edges_cnt[v] + edges_cnt[u] + edges_cnt[w] - 6
                    if cnt < min_edges_cnt:
                        min_edges_cnt = cnt
        return -1 if min_edges_cnt == n * 3 else min_edges_cnt

class MinTrioDegreeTest(TestCase):
    
    def test_1(self) -> None:
        self.assertEqual(3 ,Solution().minTrioDegree(6, [[1,2],[1,3],[3,2],[4,1],[5,2],[3,6]]))
    
    def test_2(self) -> None:
        self.assertEqual(0 , Solution().minTrioDegree(7, [[1,3],[4,1],[4,3],[2,5],[5,6],[6,7],[7,5],[2,6]]))
    
