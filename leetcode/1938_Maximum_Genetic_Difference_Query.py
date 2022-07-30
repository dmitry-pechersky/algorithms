from typing import List, Optional
from unittest import TestCase

class Node:
    def __init__(self) -> None:
        self.cnt = 0
        self.one: Optional[Node] = None
        self.zero: Optional[Node] = None

class Trie:
    def __init__(self, n: int) -> None:
        self.n = n
        self.root = Node()
    
    def insert(self, value: int) -> None:
        node = self.root
        for i in reversed(range(self.n)):
            if 0 == 1 << i & value:
                if node.zero is None:
                    node.zero = Node()
                node = node.zero
            else:
                if node.one is None:
                    node.one = Node()
                node = node.one
            node.cnt += 1

    def remove(self, value: int) -> None:
        node = self.root
        for i in reversed(range(self.n)):
            if 0 == 1 << i & value:
                node = node.zero
            else:
                node = node.one
            node.cnt -= 1

    def max_xor(self, value: int) -> int:
        node = self.root
        res = 0
        for i in reversed(range(self.n)):
            if node.one is not None and node.one.cnt > 0:
                if node.zero is not None and node.zero.cnt > 0 and 1 << i & value != 0:
                    node = node.zero
                else:
                    node = node.one
                    res += 1 << i
            else:
                node = node.zero
                
        return res ^ value
        
class Solution:
    def maxGeneticDifference(self, parents: List[int], queries: List[List[int]]) -> List[int]:
        n = len(parents)
        adj_list = [[] for i in range(n)]
        root = None
        for v, u in enumerate(parents):
            if u == -1:
                root = v
            else:
                adj_list[u].append(v)
        node_queries = [[] for i in range(n)]
        for i, (v, _) in enumerate(queries):
            node_queries[v].append(i)
        res = [0] * len(queries)

        stack = [root]
        extended = [False] * n
        trie = Trie(18)
        while stack:
            v = stack.pop()
            if extended[v]:
                trie.remove(v)
            else:
                extended[v] = True
                stack.append(v)
                for u in adj_list[v]:
                    stack.append(u)
                trie.insert(v)
                for i in node_queries[v]:
                    _, value = queries[i]
                    res[i] = trie.max_xor(value)
        return res

if __name__ == "__main__":
    parents = [-1]
    parents.extend(range(100000))
    queries = [[i, 26510] for i in range(100000)]
    Solution().maxGeneticDifference(parents, queries)
    

class MaxGeneticDifferenceTest(TestCase):
    def test_1(self):
        self.assertEqual([2,3,7], Solution().maxGeneticDifference([-1,0,1,1], [[0,2],[3,2],[2,5]]))

    def test_2(self):
        self.assertEqual([6,14,7], Solution().maxGeneticDifference([3,7,-1,2,0,7,0,2], [[4,6],[1,15],[0,5]]))

        

