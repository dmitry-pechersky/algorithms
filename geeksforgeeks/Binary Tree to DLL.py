'''
class Node:
    """ Class Node """
    def __init__(self, value):
        self.left = None
        self.data = value
        self.right = None
'''

#Your task is to complete this function
#function should return head to the DLL
def bToDLL(root):
    inorder = []
    bToDLLRec(root, inorder)
    prev = None
    for node in inorder:
        node.left = prev
        if prev is not None:
            prev.right = node
        prev = node
    prev.right = None
    return inorder[0]

def bToDLLRec(node, inorder):
    if node is None:
        return
    bToDLLRec(node.left, inorder)
    inorder.append(node)
    bToDLLRec(node.right, inorder)




