from typing import List, Dict
from unittest import TestCase

class Node:
    def __init__(self):
        self.chars = [None] * 27
        self.max_idx = None


class WordFilter:
    def __init__(self, words: List[str]):
        self.trie = Node()
        word_dic: Dict[str, int] = {}
        for idx, word in enumerate(words):
            word_dic[word] = idx
        for word, idx in word_dic.items():
            word = "{" + word
            self._append_word(self.trie, word, idx)
            for ch in word[::-1]:
                word = ch + word
                self._append_word(self.trie, word, idx)

    def f(self, prefix: str, suffix: str) -> int:
        word = suffix + "{" + prefix
        return self._search_word(self.trie, word)
                
    def _append_word(self, node: Node, word: str, idx: int):
        a_ord = ord("a")
        for ch in word:
            ch_i = ord(ch) - a_ord
            if node.chars[ch_i] is None:
                node.chars[ch_i] = Node()
            node = node.chars[ch_i]
            node.max_idx = idx

    def _search_word(self, node: Node, prefix: str) -> int:
        a_ord = ord("a")
        for ch in prefix:
            ch_i = ord(ch) - a_ord
            if node.chars[ch_i] is None:
                return -1
            node = node.chars[ch_i]
        return node.max_idx

class WordFilterTest(TestCase):
    def test_1(self):
        word_filter = WordFilter(["apple"])
        self.assertEqual(0, word_filter.f("a", "e"))

    def test_2(self):
        word_filter = WordFilter(["apple", "fruit", "appdddle"])
        self.assertEqual(2, word_filter.f("ap", "le"))
