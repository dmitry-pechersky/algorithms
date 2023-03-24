from typing import Optional
from unittest import TestCase

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def findNearestRightNode(self, root: TreeNode, u: TreeNode) -> Optional[TreeNode]:
        stack = [(root, 0)]
        target_level = None
        while stack:
            (node, level) = stack.pop()
            if target_level is None:
                if node.val == u.val:
                    target_level = level
                    continue
            else:
                if level == target_level:
                    return node
                elif level > target_level:
                    continue
            if node.right is not None:
                stack.append((node.right, level + 1))
            if node.left is not None:
                stack.append((node.left, level + 1))
        return None
        
class TestCase(TestCase):
    def test_1(self):
        u = TreeNode(4)
        target = TreeNode(5)
        self.assertEqual(
            id(target),
            id(Solution().findNearestRightNode(
                TreeNode(1, TreeNode(2, None, u), TreeNode(3, target, TreeNode(6))),
                u
            ))
        )

    def test_2(self):
        u = TreeNode(2)
        self.assertEqual(
            None,
            Solution().findNearestRightNode(
                TreeNode(3, None, TreeNode(4, u, None)),
                u
            )
        )        