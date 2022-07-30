from heapq import heapify, heappop, heappush
from typing import List
from unittest import TestCase

class DinnerPlates:

    def __init__(self, capacity: int):
        self.capacity = capacity
        self.stacks: List[List[int]] = []
        self.free_heap: List[int] = []
        self.lagerst_none_empty = -1
        
    def push(self, val: int) -> None:
        if self.free_heap:
            stack_i = heappop(self.free_heap)
            self.stacks[stack_i].append(val)
            if len(self.stacks[stack_i]) < self.capacity:
                heappush(self.free_heap, stack_i)
            if stack_i > self.lagerst_none_empty:
                self.lagerst_none_empty = stack_i
        else:
            self.stacks.append([val])
            self.lagerst_none_empty += 1
            if self.capacity > 1:
                self.free_heap.append(len(self.stacks) - 1)

    def pop(self) -> int:
        while self.lagerst_none_empty >= 0:
            if self.stacks[self.lagerst_none_empty]:
                if len(self.stacks[self.lagerst_none_empty]) == self.capacity:
                    heappush(self.free_heap, self.lagerst_none_empty)
                return self.stacks[self.lagerst_none_empty].pop()
            self.lagerst_none_empty -= 1
        return -1

    def popAtStack(self, index: int) -> int:
        if len(self.stacks) > index and self.stacks[index]:
            if len(self.stacks[index]) == self.capacity:
                heappush(self.free_heap, index)
            return self.stacks[index].pop()
        return -1

         
class DinnerPlatesTest(TestCase):
    def test_1(self):
        plates = DinnerPlates(2)
        plates.push(1)
        plates.push(2)
        plates.push(3)
        plates.push(4)
        plates.push(5)
        self.assertEqual(plates.popAtStack(0), 2)
        plates.push(20)
        plates.push(21)
        self.assertEqual(plates.popAtStack(0), 20)
        self.assertEqual(plates.popAtStack(2), 21)
        self.assertEqual(plates.pop(), 5)
        self.assertEqual(plates.pop(), 4)
        self.assertEqual(plates.pop(), 3)
        self.assertEqual(plates.pop(), 1)
        self.assertEqual(plates.pop(), -1)

    def test_2(self):
        plates = DinnerPlates(1)
        plates.push(1)
        plates.push(2)
        self.assertEqual(plates.popAtStack(1), 2)
        self.assertEqual(plates.pop(), 1)
        plates.push(1)
        plates.push(2)
        print(plates.stacks, plates.lagerst_none_empty)
        self.assertEqual(plates.pop(), 2)
        self.assertEqual(plates.pop(), 1)


