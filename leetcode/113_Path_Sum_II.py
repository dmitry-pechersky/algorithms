from typing import List, Optional
from unittest import TestCase

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def pathSum(self, root: Optional[TreeNode], targetSum: int) -> List[List[int]]:
        def rec(node: Optional[TreeNode], current_sum: int, path: List[int], paths: List[List[int]]):
            if node is not None:
                current_sum += node.val
                path.append(node.val)
                if node.left is None and node.right is None:
                    if current_sum == targetSum:
                        paths.append(path.copy())
                else:
                    rec(node.left, current_sum, path, paths)
                    rec(node.right, current_sum, path, paths)
                path.pop()

        paths = []
        rec(root, 0, [], paths)
        return paths

class PathSumTest(TestCase):
    def test_1(self) -> None:
        tree = TreeNode(
            5, 
            TreeNode(
                4, 
                TreeNode(
                    11,
                    TreeNode(7),
                    TreeNode(2)
                )
            ), 
            TreeNode(
                8,
                TreeNode(13),
                TreeNode(
                    4,
                    TreeNode(5),
                    TreeNode(1)
                )
            )
        )
        self.assertEqual(sorted(Solution().pathSum(tree, 22)), [[5,4,11,2],[5,8,4,5]])
    
    def test_2(self) -> None:
        tree = TreeNode(
            1,
            TreeNode(2),
            TreeNode(3)
        )
        self.assertEqual(Solution().pathSum(tree, 5), [])

    def test_3(self) -> None:
        tree = TreeNode(1, TreeNode(2))
        self.assertEqual(Solution().pathSum(tree, 2), [])


