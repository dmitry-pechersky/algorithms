from unittest import TestCase
from typing import Optional, List 
from collections import deque

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def levelOrderBottom(self, root: Optional[TreeNode]) -> List[List[int]]:
        res = []
        if root is not None:
            queue = deque([(root, 0)])
            while queue:
                node, level = queue.popleft()
                if level >= len(res):
                    res.append([])
                res[level].append(node.val)
                if node.left is not None:
                    queue.append((node.left, level + 1))
                if node.right is not None:
                    queue.append((node.right, level + 1))
            res.reverse()
        return res      
        
class LevelOrderBottomTest(TestCase):
    def test_1(self) -> None:
        self.assertEqual(
            Solution().levelOrderBottom(TreeNode(3, TreeNode(9), TreeNode(20, TreeNode(15), TreeNode(7)))), 
            [[15,7],[9,20],[3]]
        )

    def test_2(self) -> None:
        self.assertEqual(
            Solution().levelOrderBottom(TreeNode(1)), 
            [[1]]
        )

    def test_3(self) -> None:
        self.assertEqual(
            Solution().levelOrderBottom(None), 
            []
        )                