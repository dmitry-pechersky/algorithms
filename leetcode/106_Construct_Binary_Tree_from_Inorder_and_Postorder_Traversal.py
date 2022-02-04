from typing import List, Optional, Dict
from unittest import TestCase

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

    def __eq__(self, obj):
        return isinstance(obj, TreeNode) and self.val == obj.val and self.left == obj.left and self.right == obj.right

    def __repr__(self):
        return f'({self.val} {self.left.__repr__()} {self.right.__repr__()})'

class Solution:
    def buildTree(self, inorder: List[int], postorder: List[int]) -> Optional[TreeNode]:
        def node_rec(i_start: int, i_end: int, p: int, idxs: Dict[int, int]) -> TreeNode:
            if i_start > i_end:
                return None
            idx = idxs[postorder[p]]
            return TreeNode(
                postorder[p],
                node_rec(i_start, idx - 1, p - (i_end - idx) - 1, idxs),
                node_rec(idx + 1, i_end, p - 1, idxs)
            )
        idxs = {inorder[i]: i  for i in range(len(inorder))}
        return node_rec(0, len(inorder) - 1, len(inorder) - 1, idxs)

class BuildTreeTree(TestCase):
    def test_1(self) -> None:
        self.assertEqual(
            Solution().buildTree(
                [9,3,15,20,7], 
                [9,15,7,20,3]
            ),
            TreeNode(3, TreeNode(9), TreeNode(20, TreeNode(15), TreeNode(7)))
        )
