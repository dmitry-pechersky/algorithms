from __future__ import annotations
import unittest
from typing import Optional


class TreeNode:
    def __init__(self, val: int = 0, left: Optional[TreeNode] = None, right: Optional[TreeNode] = None) -> None:
        self.val = val
        self.left = left
        self.right = right

    def __eq__(self, o: Optional[TreeNode]) -> bool:
        return isinstance(o, TreeNode) and self.val == o.val and self.left == o.left and self.right == o.right

    def __repr__(self) -> str:
        return f"({self.val} {self.left} {self.right})"

class Solution:
    def recoverTree(self, root: Optional[TreeNode]) -> None:
        def rec(root: Optional[TreeNode], left: Optional[TreeNode], right: Optional[TreeNode]):
            if root is None:
                return True
            elif left is not None and left.val > root.val:
                left.val, root.val = root.val, left.val
                return False
            elif right is not None and right.val < root.val:
                right.val, root.val = root.val, right.val
                return False            
            return rec(root.left, left, root) and rec(root.right, root, right)

        while not rec(root, None, None):
            pass

class RecoverTreeTest(unittest.TestCase):
    def test_1(self):
        root = TreeNode(1, TreeNode(3, None, TreeNode(2)))
        res = TreeNode(3, TreeNode(1, None, TreeNode(2)))
        Solution().recoverTree(root)
        self.assertEqual(root, res)

    def test_2(self):
        root = TreeNode(3, TreeNode(1), TreeNode(4, TreeNode(2)))
        res = TreeNode(2, TreeNode(1), TreeNode(4, TreeNode(3)))
        Solution().recoverTree(root)
        self.assertEqual(root, res)
