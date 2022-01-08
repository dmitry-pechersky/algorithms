from typing import List
import unittest

class Solution:
    def partition(self, s: str) -> List[List[str]]:
        def _rec(start: int, partition: List[str], partitions: List[List[str]]) -> None:
            if start >= n:
                partitions.append(partition.copy())
            else:
                for end in range(start, n):
                    if dp[start][end]:
                        partition.append(s[start : end + 1])
                        _rec(end + 1, partition, partitions)
                        partition.pop()

        n, partition, partitions = len(s), [], []
        dp = [[False] * n for i in range(n)]
        for i in range(n):
            dp[i][i] = True
        for l in range(1, n):
            for i in range(0, n - l):
                if s[i] == s[i + l] and (l == 1 or dp[i + 1][i + l - 1]):
                    dp[i][i + l] = True
        _rec(0, partition, partitions)
        return partitions


class PartitionTest(unittest.TestCase):
    def test_1(self) -> None:
        self.assertEqual(Solution().partition("aab"), [["a","a","b"],["aa","b"]]) 

    def test_2(self) -> None:
        self.assertEqual(Solution().partition("a"), [["a"]]) 

