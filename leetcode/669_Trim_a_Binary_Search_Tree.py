from __future__ import annotations
from unittest import TestCase
from typing import Optional


class TreeNode:
    def __init__(self, val: int = 0, left: Optional[TreeNode] = None, right: Optional[TreeNode] = None):
        self.val = val
        self.left = left
        self.right = right

    def __eq__(self, o) -> bool:
        return isinstance(o, TreeNode) and self.val == o.val and self.left == o.left and self.right == o.right

    def __repr__(self) -> str:
        return f"({self.val} {self.left} {self.right})"

class Solution:
    def trimBST(self, root: Optional[TreeNode], low: int, high: int) -> Optional[TreeNode]:
        def rec(node: Optional[TreeNode]):
            if node is None:
                return None
            elif node.val > high:
                return rec(node.left)
            elif node.val < low:
                return rec(node.right)
            else:
                node.left, node.right = (rec(node.left), rec(node.right))
                return node

        return rec(root)

class TrimBSTTest(TestCase):
    def test_1(self) -> None:
        root = TreeNode(1, TreeNode(0), TreeNode(2))
        res_root = TreeNode(1, None, TreeNode(2))
        self.assertEqual(Solution().trimBST(root, 1, 2), res_root)

    def test_2(self) -> None:
        root = TreeNode(3, TreeNode(0, None, TreeNode(2, TreeNode(1))), TreeNode(4))
        res_root = TreeNode(3, TreeNode(2, TreeNode(1)))
        self.assertEqual(Solution().trimBST(root, 1, 3), res_root)        

