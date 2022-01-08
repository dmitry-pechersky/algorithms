import unittest

class Trie:
    def __init__(self):
        self.root = Node()

    def insert(self, word: str) -> None:
        node = self.root
        for idx in (ord(char) - 97 for char in word):
            if node.alp[idx] is None:
                node.alp[idx] = Node()
            node = node.alp[idx]
        node.is_word = True

    def search(self, word: str) -> bool:
        node = self.root
        for idx in (ord(char) - 97 for char in word):
            if node.alp[idx] is None:
                return False
            node = node.alp[idx]
        return node.is_word

    def startsWith(self, prefix: str) -> bool:
        node = self.root
        for idx in (ord(char) - 97 for char in prefix):
            if node.alp[idx] is None:
                return False
            node = node.alp[idx]
        return True

class Node: 
    def __init__(self):
        self.alp = [None] * 26
        self.is_word = False

class TestTrie(unittest.TestCase):
    def test_1(self) -> None:
        trie = Trie()
        trie.insert("apple")
        self.assertTrue(trie.search("apple"))
        self.assertFalse(trie.search("app"))
        self.assertTrue(trie.startsWith("app"))
        trie.insert("app")
        self.assertTrue(trie.search("app"))
        
