from unittest import TestCase

class Node: 
    def __init__(self, value: int):
        self.value = value
        self.next = None
        self.prev = None

class DLList:
    def __init__(self):
        node = Node(None)
        node.next, node.prev = node, node
        self.head = node

    def left_push(self, node: Node) -> None:
        head, next_node = self.head, self.head.next
        head.next, next_node.prev = node, node
        node.next, node.prev = next_node, head
        
    def pop(self, node: Node) -> Node:
        prev_node, next_node = node.prev, node.next
        prev_node.next = next_node
        next_node.prev = prev_node
        node.prev, node.next = None, None
        return node

    def right_pop(self) -> Node:
        return self.pop(self.head.prev)

    def __repr__(self) -> str:
        current = self.head.next
        arr = []
        while current.value is not None:
            arr.append(current.value)
            current = current.next
        return arr.__repr__()

class LRUCache:
    def __init__(self, capacity: int):
        self.free = capacity
        self.dic = {}
        self.dllist = DLList()
        
    def get(self, key: int) -> int:
        value, node = self.dic.get(key, (-1, None))
        if node is not None:
            self.dllist.pop(node)
            self.dllist.left_push(node)
        return value

    def put(self, key: int, value: int) -> None:
        _, node = self.dic.get(key, (-1, None))
        if node is None:
            if self.free == 0:
                self.dic.pop(self.dllist.right_pop().value)
            else:
                self.free -= 1
            node = Node(key)
        else:
            self.dllist.pop(node)
        self.dic[key] = (value, node)
        self.dllist.left_push(node)

class LRUCacheTest(TestCase):
    def test_1(self) -> None:
        cache = LRUCache(2)
        cache.put(1, 1)
        cache.put(2, 2)
        self.assertEqual(cache.get(1), 1)        
        cache.put(3, 3)
        self.assertEqual(cache.get(2), -1)
        cache.put(4, 4)
        self.assertEqual(cache.get(1), -1)
        self.assertEqual(cache.get(3), 3)
        self.assertEqual(cache.get(4), 4)

    def test_2(self) -> None:
        cache = LRUCache(3)
        cache.put(1,1) 
        cache.put(2,2)
        cache.put(3,3)
        cache.put(4,4) 
        self.assertEqual(cache.get(4), 4)
        self.assertEqual(cache.get(3), 3)
        self.assertEqual(cache.get(2), 2)
        self.assertEqual(cache.get(1), -1)
        cache.put(5, 5)
        self.assertEqual(cache.get(1), -1)
        self.assertEqual(cache.get(2), 2)
        self.assertEqual(cache.get(3), 3)
        self.assertEqual(cache.get(4), -1)
        self.assertEqual(cache.get(5), 5)

