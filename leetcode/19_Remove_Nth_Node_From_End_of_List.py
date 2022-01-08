import unittest
from typing import List, Optional

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

    def __eq__(self, other):
        if isinstance(other, ListNode):
            return self.val == other.val and self.next == other.next
        return False

class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        lag, current = head, head
        for i in range(n):
            current = current.next
        if current is None:
            return lag.next
        while current.next is not None:
            current = current.next
            lag = lag.next
        lag.next = lag.next.next
        return head

class TestRemoveNthFromEnd(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().removeNthFromEnd(ListNode(1), 1), None)    

    def test_2(self):
        self.assertEqual(Solution().removeNthFromEnd(ListNode(1, ListNode(2)), 2), ListNode(2))    

    def test_3(self):
        self.assertEqual(Solution().removeNthFromEnd(ListNode(1, ListNode(2)), 1), ListNode(1))    

    def test_4(self):
        self.assertEqual(
            Solution().removeNthFromEnd(
                ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5))))), 
                2), 
            ListNode(1, ListNode(2, ListNode(3, ListNode(5)))))    
