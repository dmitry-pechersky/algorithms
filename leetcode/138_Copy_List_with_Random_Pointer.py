class Node:
    def __init__(self, x, next = None, random = None):
        self.val = int(x)
        self.next = next
        self.random = random

class Solution:
    def copyRandomList(self, head ):
        if head is None:
            return None
        dic = {}
        current = head
        while current is not None:
            dic[id(current)] = Node(current.val)
            current = current.next
        current = head
        while current is not None:
            node = dic[id(current)]
            if current.next is not None:
                node.next = dic[id(current.next)]
            if current.random is not None:
                node.random = dic[id(current.random)]
            current = current.next
        return dic[id(head)]
