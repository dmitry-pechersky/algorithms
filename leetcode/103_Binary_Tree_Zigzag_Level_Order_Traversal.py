from typing import List, Optional
import unittest

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

    def __repr__(self) -> str:
        return str(self.val)

class Solution:
    def zigzagLevelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        current_stack, next_stack = [root], []
        level = 0
        res = []
        while current_stack:
            node = current_stack.pop()
            if node is not None:
                while level >= len(res):
                    res.append([])
                res[level].append(node.val)
                if level % 2 == 0:
                    next_stack.append(node.left)
                    next_stack.append(node.right)
                else:
                    next_stack.append(node.right)
                    next_stack.append(node.left)
            if not current_stack:
                current_stack, next_stack = next_stack, current_stack
                level += 1
        return res

class ZigzagLevelOrderTest(unittest.TestCase):
    def test_1(self) -> None:
        self.assertEqual(Solution().zigzagLevelOrder(TreeNode(3, TreeNode(9), TreeNode(20, TreeNode(15), TreeNode(7)))), [[3],[20,9],[15,7]])
    
    def test_2(self) -> None:
        self.assertEqual(Solution().zigzagLevelOrder(TreeNode(1)), [[1]])

    def test_3(self) -> None:
        self.assertEqual(Solution().zigzagLevelOrder(None), [])
    
