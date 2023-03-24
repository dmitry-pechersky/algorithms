import unittest
from typing import Optional, List

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

    @classmethod
    def list_to_listnode(cls, lis: List[int]) -> Optional[ListNode]:
        if len(lis) > 0:
            return ListNode(lis[0], cls.list_to_listnode(lis[1:]))
        return None


    def test_1(self) -> None:
        self.assertEqual(
            Solution().deleteDuplicates(self.list_to_listnode([1,2,3,3,4,4,5])),
            self.list_to_listnode([1,2,5])
        )

    def test_2(self) -> None:
        self.assertEqual(
            Solution().deleteDuplicates(self.list_to_listnode([1,1,1,2,3])),
            self.list_to_listnode([2,3])
        )

