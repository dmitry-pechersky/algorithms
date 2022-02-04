from unittest import TestCase
from typing import Dict

class WordDictionary:
    def __init__(self):
        self.root = {}

    def addWord(self, word: str) -> None:
        current = self.root
        for ch in word:
            if ch not in current:
                current[ch] = {}
            current = current[ch]
        current['$'] = {}

    def search(self, word: str) -> bool:
        return self._search_rec_(self.root, word + '$', 0)

    def _search_rec_(self, node: Dict[str, Dict], word: str, i: int) -> bool:
        ch = word[i]
        if ch == '$':
            return '$' in node
        elif ch == '.':
            for next_node in node.values():
                if self._search_rec_(next_node, word, i + 1):
                    return True
            return False
        elif ch in node:
            return self._search_rec_(node[ch], word, i + 1)
        return False

class WordDictionaryTest(TestCase):
    def test_1(self) -> None:
        wordDictionary = WordDictionary()
        wordDictionary.addWord("bad")
        wordDictionary.addWord("dad")
        wordDictionary.addWord("mad")
        self.assertFalse(wordDictionary.search("pad"))
        self.assertTrue(wordDictionary.search("bad"))
        self.assertTrue(wordDictionary.search(".ad"))
        self.assertTrue(wordDictionary.search("b.."))
