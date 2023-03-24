from unittest import TestCase

from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

    def __eq__(self, o):
        return isinstance(o, TreeNode) and self.val == o.val and self.left == o.left and self.right == o.right

class Solution:
    def deleteNode(self, root: Optional[TreeNode], key: int) -> Optional[TreeNode]:
        def delete_node(node: Optional[TreeNode], key: int):
            if node is not None:
                if node.val < key:
                    node.right = delete_node(node.right, key)
                elif node.val > key:
                    node.left = delete_node(node.left, key)
                else:
                    if node.left is None:
                        node = node.right
                    elif node.right is None:
                        node = node.left
                    else:
                        least_node = node.right
                        while least_node.left is not None:
                            least_node = least_node.left
                        node.val = least_node.val
                        node.right = delete_node(node.right, node.val)
            return node

        return delete_node(root, key)

class DeleteNodeTest(TestCase):
    def test_1(self):
        self.assertEqual(
            Solution().deleteNode(TreeNode(5, TreeNode(3, TreeNode(2), TreeNode(4)), TreeNode(6, None, TreeNode(7))), 3), 
            TreeNode(5, TreeNode(4, TreeNode(2), None), TreeNode(6, None, TreeNode(7)))
        )