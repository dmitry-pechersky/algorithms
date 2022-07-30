from typing import List
from unittest import TestCase
from itertools import combinations
from collections import deque

class Solution:
    def numBusesToDestination(self, routes: List[List[int]], source: int, target: int) -> int:
        if source == target:
            return 0
        bus_n = len(routes)
        stop_bus_dict = {}
        adj_list = [[] for i in range(bus_n)]

        for bus in range(bus_n):
            for stop in routes[bus]:
                if not stop in stop_bus_dict:
                    stop_bus_dict[stop] = []
                stop_bus_dict[stop].append(bus)

        for buses in stop_bus_dict.values():
            for v, u in combinations(buses, 2):
                adj_list[v].append(u)
                adj_list[u].append(v)

        if target in stop_bus_dict:
            targets = set(stop_bus_dict[target])
            queue = deque((bus, 1) for bus in stop_bus_dict[source])
            expanded = [False] * bus_n
            while queue:
                v, cost = queue.popleft()
                if not expanded[v]:
                    expanded[v] = True
                    if v in targets:
                        return cost
                    for u in adj_list[v]:
                        if not expanded[u]:
                            queue.append((u, cost + 1))
        return -1

        

class NumBusesToDestinationTest(TestCase):
    def test_1(self) -> None:
        self.assertEqual(2, Solution().numBusesToDestination([[1,2,7],[3,6,7]], 1, 6))

    def test_2(self) -> None:
        self.assertEqual(-1, Solution().numBusesToDestination([[7,12],[4,5,15],[6],[15,19],[9,12,13]], 15, 12))        