import unittest
from typing import Optional, List

class TreeNode():
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution():
    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        stack = [(root, False)]
        inorder = []
        while stack:
            node, expanded = stack.pop()
            if node is not None:
                if expanded:
                    inorder.append(node.val)
                else:
                    stack.append((node.right, False))
                    stack.append((node, True))
                    stack.append((node.left, False))
        return inorder

class InorderTraversalTest(unittest.TestCase):
    def test_1(self) -> None:
        self.assertEqual(
            Solution().inorderTraversal(
                TreeNode(1, None, TreeNode(2, TreeNode(3), None))
            ),
            [1,3,2]
        )

    def test_2(self) -> None:
        self.assertEqual(Solution().inorderTraversal(10), [])

    def test_3(self) -> None:
        self.assertEqual(Solution().inorderTraversal(TreeNode(1)), [1])

    def test_4(self) -> None:
        self.assertEqual(Solution().inorderTraversal(TreeNode(1, TreeNode(2), None)), [2, 1])

    def test_5(self) -> None:
        self.assertEqual(Solution().inorderTraversal(TreeNode(1, None, TreeNode(2))), [1, 2])
