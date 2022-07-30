from typing import List
from unittest import TestCase


class DisjointSet:
    def __init__(self, n: int) -> None:
        self.parents = [i for i in range(n)]
        self.ranks = [0] * n

    def find(self, i: int) -> int:
        while i != self.parents[i]:
            i, self.parents[i] = self.parents[i], self.parents[self.parents[i]]
        return i

    def join(self, i: int, j: int) -> None:
        root_i, root_j = self.find(i), self.find(j)
        if root_i != root_j:
            if self.ranks[root_i] < self.ranks[root_j]:
                self.parents[root_i] = root_j
            elif self.ranks[root_i] > self.ranks[root_j]:
                self.parents[root_j] = root_i
            else:
                self.parents[root_i] = root_j
                self.ranks[root_j] += 1

class Solution:
    def smallestStringWithSwaps(self, s: str, pairs: List[List[int]]) -> str:
        n = len(s)
        ds = DisjointSet(n)
        res_str = [None] * n
        for i, j in pairs:
            ds.join(i, j)
        groups = {}
        for i, ch in enumerate(s):
            root = ds.find(i)
            if root not in groups:
                groups[root] = ([], [])
            indexes, chars = groups[root]
            indexes.append(i)
            chars.append(ch)
        for indexes, chars in groups.values():
            chars.sort()
            for index, ch in zip(indexes, chars):
                res_str[index] = ch
        return "".join(res_str)

class SmallestStringWithSwaps(TestCase):
    def test_1(self):
        self.assertEqual(Solution().smallestStringWithSwaps("dcab", [[0,3],[1,2]]), "bacd")

    def test_2(self):
        self.assertEqual(Solution().smallestStringWithSwaps("dcab", [[0,3],[1,2],[0,2]]), "abcd")        

    def test_3(self):
        self.assertEqual(Solution().smallestStringWithSwaps("cba", [[0,1],[1,2]]), "abc")                