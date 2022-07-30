from unittest import TestCase
from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

    def __eq__(self, obj) -> bool:
        return isinstance(obj, TreeNode) and self.val == obj.val and self.left == obj.left and self.right == obj.right

    def __repr__(self) -> str:
        return f'({self.val} {self.left} {self.right})'

class Solution:
    def flatten(self, root: Optional[TreeNode]) -> None:
        if root is not None:
            stack = [root]
            last = TreeNode(0, None, None)
            while stack:
                node = stack.pop()
                if node.right is not None:
                    stack.append(node.right)
                if node.left is not None:
                    stack.append(node.left)
                node.left, node.right = None, None
                last.right = node
                last = node

class FlattenTest(TestCase):
    def test_1(self):
        root = TreeNode(1, TreeNode(2, TreeNode(3), TreeNode(4)), TreeNode(5, right=TreeNode(6)))
        target = TreeNode(1, right=TreeNode(2, right=TreeNode(3, right=TreeNode(4, right=TreeNode(5, right=TreeNode(6))))))
        Solution().flatten(root)
        self.assertEqual(root, target)

    def test_2(self):
        root = None 
        Solution().flatten(root)
        self.assertEqual(root, None)

    def test_3(self):
        root = TreeNode(0)
        Solution().flatten(root)
        self.assertEqual(root, TreeNode(0))
