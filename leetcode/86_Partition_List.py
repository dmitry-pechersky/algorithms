from typing import Any, Optional
import unittest

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

    def __eq__(self, o: Any) -> bool:
        if not isinstance(o, ListNode):
            return False
        return self.val == o.val and self.next == o.next

    def __repr__(self) -> str:
        l = []
        current = self
        while current is not None:
            l.append(current.val)
            current = current.next
        return l.__repr__()

class Solution:
    def partition(self, head: Optional[ListNode], x: int) -> Optional[ListNode]:
        left_head, right_head = ListNode(0), ListNode(0)
        left_tail, right_tail = left_head, right_head
        current = head
        while current is not None:
            node = current
            current = current.next
            node.next = None
            if node.val < x:
                left_tail.next = node
                left_tail = node
            else :
                right_tail.next = node
                right_tail = node
        left_tail.next = right_head.next
        return left_head.next

class PartitionTest(unittest.TestCase):
    def test_1(self) -> None:
        self.assertEqual(
            Solution().partition(
                ListNode(1,ListNode(4,ListNode(3,ListNode(2,ListNode(5,ListNode(2)))))),
                3
            ),
            ListNode(1,ListNode(2,ListNode(2,ListNode(4,ListNode(3,ListNode(5))))))
        )

    def test_2(self) -> None:
        self.assertEqual(
            Solution().partition(
                ListNode(2,ListNode(1)),
                2
            ),
            ListNode(1,ListNode(2))
        )
