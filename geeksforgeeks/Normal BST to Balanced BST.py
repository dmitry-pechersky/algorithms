"""
class Node:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None
"""
def inOrderRec(root, array):
    if root is not None:
        inOrderRec(root.left, array)
        array.append(root.data)
        inOrderRec(root.right, array)        

def arrayToBinaryTreeRec(array, start, end):
    if start > end:
        return None
    mid = (start + end) // 2
    node = Node(array[mid])
    node.left = arrayToBinaryTreeRec(array, start, mid - 1)
    node.right = arrayToBinaryTreeRec(array, mid + 1, end)
    return node

def buildBalancedTree(root): 
    array = []
    inOrderRec(root, array)
    return arrayToBinaryTreeRec(array, 0, len(array) - 1)
