from typing import Optional, Tuple, List
from unittest import TestCase

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def deepestLeavesSum(self, root: Optional[TreeNode]) -> int:
        max_level, max_level_sum = 0, 0
        if root is not None:
            stack = [(0, root)]
            while stack:
                level, v = stack.pop()
                if max_level < level:
                    max_level, max_level_sum = level, v.val
                elif level == max_level:
                    max_level_sum += v.val
                if v.left is not None:
                    stack.append((level + 1, v.left))
                if v.right is not None:
                    stack.append((level + 1, v.right))
        return max_level_sum

class DeepestLeavesSumTest(TestCase):
    def test_1(self):
        self.assertEqual(15, Solution().deepestLeavesSum(TreeNode(1, TreeNode(2, TreeNode(4, TreeNode(7)), TreeNode(5)), TreeNode(3, None, TreeNode(6, None, TreeNode(8))))))

            