class Node:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

def array_to_bst(array, start, end):
    if start > end:
        return None
    mid = (end + start) // 2
    node = Node(array[mid])
    node.left = array_to_bst(array, start, mid - 1)
    node.right = array_to_bst(array, mid + 1, end)
    return node

def preorder(node, order):
    if node is not None:
        order.append(node.data)
        preorder(node.left, order)
        preorder(node.right, order)

if __name__ == '__main__':
    for t in range(int(input())):
        n, array = int(input()), [int(i) for i in input().split()]
        order = []
        preorder(array_to_bst(array, 0, n - 1), order)
        print(*order)
