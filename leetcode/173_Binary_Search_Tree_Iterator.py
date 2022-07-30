from __future__ import annotations
from typing import Optional
from unittest import TestCase

class TreeNode:
    def __init__(self, val=0, left: Optional[TreeNode] = None, right: Optional[TreeNode] = None) -> None:
        self.val = val
        self.left = left
        self.right = right

class BSTIterator:

    def __init__(self, root: Optional[TreeNode]):
        self.stack = []
        if root is not None:
            self.stack.append((root, False))
        
    def next(self) -> int:
        while True:
            node, expanded = self.stack.pop()
            if expanded:
                return node.val
            else:
                if node.right is not None:
                    self.stack.append((node.right, False))
                self.stack.append((node, True))
                if node.left is not None:
                    self.stack.append((node.left, False))

    def hasNext(self) -> bool:
        return len(self.stack) > 0
        
class BSTIteratorTest(TestCase):
    def test_1(self) -> None:
        itr = BSTIterator(TreeNode(7, TreeNode(3), TreeNode(15, TreeNode(9), TreeNode(20))))
        self.assertEqual(itr.next(), 3)
        self.assertEqual(itr.next(), 7)
        self.assertTrue(itr.hasNext())
        self.assertEqual(itr.next(), 9)
        self.assertTrue(itr.hasNext())
        self.assertEqual(itr.next(), 15)
        self.assertTrue(itr.hasNext())
        self.assertEqual(itr.next(), 20)
        self.assertFalse(itr.hasNext())
        
