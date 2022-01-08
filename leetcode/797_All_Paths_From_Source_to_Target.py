import unittest
from typing import List

class Solution:
    def allPathsSourceTarget(self, graph: List[List[int]]) -> List[List[int]]:
        n = len(graph)
        path, paths = [], []
        def _rec(node: int) -> None:
            path.append(node)
            if node == n - 1:
                paths.append(path.copy())
            else:
                for successor in graph[node]:
                    _rec(successor)
            path.pop()
        _rec(0)
        return paths
        
class AllPathsSourceTargetTest(unittest.TestCase):
    def test_1(self) -> None:
        self.assertEqual(
            sorted(Solution().allPathsSourceTarget([[1,2],[3],[3],[]])),
            sorted([[0,1,3],[0,2,3]])
            )    

    def test_2(self) -> None:
        self.assertEqual(
            sorted(Solution().allPathsSourceTarget([[4,3,1],[3,2,4],[3],[4],[]])),
            sorted([[0,4],[0,3,4],[0,1,3,4],[0,1,2,3,4],[0,1,4]])
            )    

    def test_3(self) -> None:
        self.assertEqual(
            sorted(Solution().allPathsSourceTarget([[1],[]])),
            sorted([[0,1]])
            )    

    def test_4(self) -> None:
        self.assertEqual(
            sorted(Solution().allPathsSourceTarget([[1,2,3],[2],[3],[]])),
            sorted([[0,1,2,3],[0,2,3],[0,3]])
            )    

    def test_5(self) -> None:
        self.assertEqual(
            sorted(Solution().allPathsSourceTarget([[1,3],[2],[3],[]])),
            sorted([[0,1,2,3],[0,3]])
            )    

