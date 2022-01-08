import unittest
from typing import List

class Solution:
    def findOrder(self, numCourses: int, prerequisites: List[List[int]]) -> List[int]:
        def dfs(u: int, adj_list: List[List[int]], in_stack: List[bool], visited: List[bool], order: List[int]) -> None:
            visited[u] = True
            in_stack[u] = True
            for v in adj_list[u]:
                if in_stack[v]:
                    return False
                if not visited[v]:
                    if not dfs(v, adj_list, in_stack, visited, order):
                        return False
            in_stack[u] = False
            order.append(u)
            return True

        adj_list = [[] for i in range(numCourses)]
        for u, v in prerequisites:
            adj_list[u].append(v)
        order = []
        in_stack, visited = [False] * numCourses, [False] * numCourses
        for u in range(numCourses):
            if not visited[u]:
                if not dfs(u, adj_list, in_stack, visited, order):
                    return []
        return order

class FindOrderTest(unittest.TestCase):
    def test_1(self) -> None:
        self.assertEqual(Solution().findOrder(2, [[1,0]]), [0,1])

    def test_2(self) -> None:
        self.assertIn(Solution().findOrder(4, [[1,0],[2,0],[3,1],[3,2]]), [[0,1,2,3], [0,2,1,3]])        

    def test_3(self) -> None:
        self.assertEqual(Solution().findOrder(1, []), [0])        

    def test_4(self) -> None:
        self.assertEqual(Solution().findOrder(7, [[1,0],[0,3],[0,2],[3,2],[2,5],[4,5],[5,6],[2,4]]), [6,5,4,2,3,0,1])                

    def test_5(self) -> None:
        self.assertEqual(Solution().findOrder(7, [[1,0],[0,3],[0,2],[3,2],[2,5],[4,5],[5,6],[2,4], [4,3]]), [])                