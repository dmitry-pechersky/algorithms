from  typing import List, Optional
import unittest 

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        order = []
        if root is not None:
            stack = [(root, 0)]
            while stack:
                node, level = stack.pop()
                while level >= len(order):
                    order.append([])
                order[level].append(node.val)
                if node.right is not None:
                    stack.append((node.right, level + 1))
                if node.left is not None:
                    stack.append((node.left, level + 1))
        return order

class LevelOrderTest(unittest.TestCase):
    def test_1(self) -> None:
        self.assertEqual(
            Solution().levelOrder(TreeNode(3, TreeNode(9), TreeNode(20, TreeNode(15), TreeNode(7)))),
            [[3],[9,20],[15,7]]
            )

    def test_2(self) -> None:
        self.assertEqual(
            Solution().levelOrder(TreeNode(1)),
            [[1]]
            )

    def test_3(self) -> None:
        self.assertEqual(
            Solution().levelOrder(None),
            []
            )
        