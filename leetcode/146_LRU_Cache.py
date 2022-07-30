from unittest import TestCase
from collections import OrderedDict

class LRUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.dic = OrderedDict()
        
    def get(self, key: int) -> int:
        if key in self.dic:
            self.dic.move_to_end(key)
            return self.dic[key]
        return -1

    def put(self, key: int, value: int) -> None:
        if key in self.dic:
            self.dic[key] = value
            self.dic.move_to_end(key)
        else:
            if len(self.dic) >= self.capacity:
                self.dic.popitem(last=False)
            self.dic[key] = value

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
        
    def test_3(self) -> None:
        cache = LRUCache(2)
        self.assertEqual(cache.get(2), -1)
        cache.put(2, 6)
        self.assertEqual(cache.get(1), -1)
        cache.put(1, 5)
        cache.put(1, 2)
        self.assertEqual(cache.get(1), 2)
        self.assertEqual(cache.get(2), 6)
