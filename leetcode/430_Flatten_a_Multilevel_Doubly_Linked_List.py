class Node:
    def __init__(self, val, prev, next, child):
        self.val = val
        self.prev = prev
        self.next = next
        self.child = child

class Solution:
    def flatten(self, head):
        res, tail = None, None
        if head is not None:
            stack = [head]
            while stack:
                node = stack.pop()
                if node.next is not None:
                    stack.append(node.next)
                if node.child is not None:
                    stack.append(node.child)
                node.prev, node.child, node.next = tail, None, None
                if res is None:
                    res = node
                else:
                    tail.next = node
                tail = node
        return res
