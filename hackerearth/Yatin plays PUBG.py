class Node:
    def __init__(self, value):
        self.value = value
        self.left, self.right = None, None

def append(node, value):
    if value > node.value:
        if node.right is None:
            node.right = Node(value)
        else:
            append(node.right, value)
    else:
        if node.left is None:
            node.left = Node(value)
        else:
            append(node.left, value)

def depth(node, d):
    if node is None:
        return d
    return max(depth(node.left, d + 1), depth(node.right, d + 1))


if __name__ == '__main__':
    for t in range(int(input())):
        n = int(input())
        values = [int(i) for i in input().split()]
        node = Node(values[0])
        for i in range(1, n):
            append(node, values[i])
        print(depth(node, 0))