from typing import Optional
import unittest

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def maxAncestorDiff(self, root: Optional[TreeNode]) -> int:
        diff = 0
        stack = [(root, root.val, root.val)]
        while stack:
            node, min_value, max_value = stack.pop()
            if abs(min_value - node.val) > diff:
                diff = abs(min_value - node.val)
            if abs(max_value - node.val) > diff:
                diff = abs(max_value - node.val)
            if node.val < min_value:
                min_value= node.val
            elif node.val > max_value:
                max_value = node.val
            if node.left is not None:
                stack.append((node.left, min_value, max_value))
            if node.right is not None:
                stack.append((node.right, min_value, max_value))
        return diff

    def _maxAncestorDiff(self, root: Optional[TreeNode]) -> int:
        diff = 0
        def _rec(node: Optional[TreeNode], min_value: int, max_value: int):
            nonlocal diff
            if node is not None:
                if abs(min_value - node.val) > diff:
                    diff = abs(min_value - node.val)
                if abs(max_value - node.val) > diff:
                    diff = abs(max_value - node.val)
                if node.val < min_value:
                    min_value= node.val
                elif node.val > max_value:
                    max_value = node.val
                _rec(node.left, min_value, max_value)
                _rec(node.right, min_value, max_value)
        _rec(root, root.val, root.val)
        return diff

class MaxAncestorDiff(unittest.TestCase):
    def test_1(self) -> None:
        self.assertEqual(
            Solution().maxAncestorDiff(
                TreeNode(8, TreeNode(3, TreeNode(1, TreeNode(6, TreeNode(4), TreeNode(7)))), TreeNode(10, None, TreeNode(14, TreeNode(13))))
            ),
            7
        )

    def test_2(self) -> None:
        self.assertEqual(
            Solution().maxAncestorDiff(
                TreeNode(1, None, TreeNode(2, None, TreeNode(0, TreeNode(3))))
            ),
            3
        )
