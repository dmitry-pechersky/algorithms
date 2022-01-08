from typing import List, Optional
import unittest

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

    def __eq__(self, obj):
        return isinstance(obj, TreeNode) and self.val == obj.val and self.left == obj.left and self.right == obj.right

    def __repr__(self):
        return f"({self.val} {self.left} {self.right})"

class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        def _rec(in_i: int, in_j: int) -> Optional[TreeNode]:
            if  in_i > in_j:
                return None
            nonlocal current_val_idx
            val = preorder[current_val_idx]
            inorder_val_idx = inorder_idx[val]
            current_val_idx += 1
            return TreeNode(
                val,
                _rec(in_i, inorder_val_idx - 1),
                _rec(inorder_val_idx + 1, in_j)
            )
        inorder_idx = {value: idx for idx, value in enumerate(inorder)}
        n = len(preorder)
        current_val_idx = 0
        return _rec(0, n - 1)

class BuildTreeTest(unittest.TestCase):

    def test_1(self) -> None:
        self.assertEqual(
            Solution().buildTree([3,9,20,15,7], [9,3,15,20,7]),
            TreeNode(3, TreeNode(9), TreeNode(20, TreeNode(15), TreeNode(7)))
            ) 

    def test_2(self) -> None:
        self.assertEqual(
            Solution().buildTree([-1], [-1]),
            TreeNode(-1)
            ) 
    def test_3(self) -> None:
        self.assertEqual(
            Solution().buildTree([1, 2], [2, 1]),
            TreeNode(1, TreeNode(2))
            ) 
