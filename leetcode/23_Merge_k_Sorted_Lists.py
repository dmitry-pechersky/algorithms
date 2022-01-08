import unittest
from typing import List, Optional
from heapq import heapify, heappop, heappush

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
    
    def __eq__(self, other):
        if isinstance(other, ListNode):
            return self.val == other.val and self.next == other.next
        return false

def list_node_lt(self, other):
    return self.val < other.val

ListNode.__lt__ =  list_node_lt

class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        queue = [node for node in lists if node is not None]
        if len(queue) == 0:
            return None
        heapify(queue)
        head, tail = None, None 
        while len(queue) > 0:
            node = heappop(queue)
            if node.next is not None:
                heappush(queue, node.next)
                node.next = None
            if head is None:
                head, tail = node, node
            else:
                tail.next = node
                tail = node
        return head

class TestMergeKLists(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().mergeKLists(
            [
                ListNode(1, ListNode(4, ListNode(5))),
                ListNode(1, ListNode(3, ListNode(4))),
                ListNode(2, ListNode(6)),
            ]), 
            ListNode(1, ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(4, ListNode(5, ListNode(6)))))))))    

    def test_2(self):
        self.assertEqual(Solution().mergeKLists([]), None)    

    def test_3(self):
        self.assertEqual(Solution().mergeKLists([None]), None)    

    def test_4(self):
        self.assertEqual(Solution().mergeKLists([ListNode(1)]), ListNode(1))    


