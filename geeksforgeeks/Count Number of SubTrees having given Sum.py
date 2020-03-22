'''
# Node Class:
class Node:
    def __init__(self,val):
        self.data = val
        self.left = None
        self.right = None
'''

def countSubtreesWithSumX(root ,x):
    '''
    :param root: root of given tree
    :param x: value of x.
    :return: Count of such subtrees
    '''
    # code here
    _, cnt = countSubtreesWithSumXRec(root ,x)
    return cnt

def countSubtreesWithSumXRec(root, x):
    if root is None:
        return (0,0)
    left_sum, left_cnt = countSubtreesWithSumXRec(root.left, x)
    right_sum, right_cnt = countSubtreesWithSumXRec(root.right, x)
    s = left_sum + right_sum + root.data
    cnt = left_cnt + right_cnt + (1 if s == x else 0)
    return s, cnt
