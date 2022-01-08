import unittest
from typing import Optional

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        return self.val == other.val and self.next == other.next

    def __repr__(self):
        vals, current = [], self
        while current is not None:
            vals.append(current.val)
            current = current.next
        return vals.__repr__()

class Solution:
    def rotateRight(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        if head is None:
            return head
        n, tail = 1, head
        while tail.next is not None:
            tail = tail.next
            n += 1
        k = k % n
        if k == 0:
            return head
        new_tail = head
        while k + 1 < n:
            new_tail = new_tail.next
            k += 1
        new_head = new_tail.next
        new_tail.next = None
        tail.next = head

        return new_head

class TestRotateRight(unittest.TestCase):
    def test_1(self):
        self.assertEqual(
            Solution().rotateRight(ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5))))), 2),
            ListNode(4, ListNode(5, ListNode(1, ListNode(2, ListNode(3)))))
            )

    def test_2(self):
        self.assertEqual(
            Solution().rotateRight(ListNode(0, ListNode(1, ListNode(2))), 4),
            ListNode(2, ListNode(0, ListNode(1)))
            )

    def test_3(self):
        self.assertEqual(
            Solution().rotateRight(ListNode(1), 28),
            ListNode(1))
