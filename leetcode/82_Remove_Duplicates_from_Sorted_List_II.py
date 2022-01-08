import unittest
from typing import Optional

class ListNode:
    def __init__(self, val=0, next=None) -> None:
        self.val = val
        self.next = next

    def __eq__(self, other) -> bool:
        if not isinstance(other, self.__class__):
            return False
        return self.val == other.val and self.next == other.next

    def __repr__(self) -> str:
        vals = []
        current = self
        while current is not None:
            vals.append(current.val)
            current = current.next
        return vals.__repr__()

class Solution:
    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        dummy_head = ListNode(None, head)
        current = dummy_head
        delete_val = None
        while current.next is not None:
            if current.next.val == delete_val:
                current.next = current.next.next
            elif current.next.next is not None and current.next.val == current.next.next.val:
                delete_val = current.next.val
                current.next = current.next.next
            else:
                current = current.next
        return dummy_head.next

class TestDeleteDuplicates(unittest.TestCase):
    def test_1(self) -> None:
        self.assertEqual(
            Solution().deleteDuplicates(
                ListNode(1, ListNode(2, (ListNode(3 ,ListNode(3 ,ListNode(4, ListNode(4, ListNode(5))))))))
                ),
            ListNode(1, ListNode(2, ListNode(5)))
        )

    def test_2(self) -> None:
        self.assertEqual(
            Solution().deleteDuplicates(
                ListNode(1 ,ListNode(1 ,ListNode(1, ListNode(2, ListNode(3)))))
                ),
            ListNode(2, ListNode(3))
        )
