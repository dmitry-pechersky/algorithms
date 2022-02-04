from unittest import TestCase
from typing import List, Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def getAllElements(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> List[int]:
        def inorder_rec(root: Optional[TreeNode], inorder: List[int]) -> None:
            if root is not None:
                inorder_rec(root.left, inorder)
                inorder.append(root.val)
                inorder_rec(root.right, inorder)

        def merge(array1: List[int], array2: List[int]) -> List[int]:
            array = []
            n1, n2 = len(array1), len(array2)
            i1, i2 = 0, 0
            while i1 < n1 or i2 < n2:
                if i1 < n1 and i2 < n2:
                    if array1[i1] < array2[i2]:
                        array.append(array1[i1])
                        i1 += 1
                    else:
                        array.append(array2[i2])
                        i2 += 1
                elif i1 < n1:
                    array.append(array1[i1])
                    i1 += 1
                elif i2 < n2:
                    array.append(array2[i2])
                    i2 += 1
            return array
                
        inorder1, inorder2 = [], []
        inorder_rec(root1, inorder1)
        inorder_rec(root2, inorder2)
        return merge(inorder1, inorder2)

class GetAllElementsTest(TestCase):
    def test_1(self) -> None:
        self.assertEqual(
            Solution().getAllElements(
                TreeNode(2, TreeNode(1), TreeNode(4)),
                TreeNode(1, TreeNode(0), TreeNode(3))
            ), 
            [0,1,1,2,3,4]
        )

    def test_2(self) -> None:
        self.assertEqual(
            Solution().getAllElements(
                TreeNode(1, None, TreeNode(8)),
                TreeNode(8, TreeNode(1), None)
            ), 
            [1,1,8,8]
        )
