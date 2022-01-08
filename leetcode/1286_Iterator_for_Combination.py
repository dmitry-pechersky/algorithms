import unittest

class CombinationIterator:

    def __init__(self, characters: str, combinationLength: int):
        self.characters = characters
        self.combination_length = combinationLength
        self.has_next = True
        self.combination  = list(range(self.combination_length))
    
    def next(self) -> str:
        res = "".join(self.characters[i] for i in self.combination)
        n = len(self.characters)
        i = self.combination_length - 1
        while i >= 0 and self.combination[i] == n - 1 - (self.combination_length  - 1 - i):
            i -= 1
        if i == -1:
            self.has_next = False
        else:
            self.combination[i] += 1
            for j in range(i + 1, self.combination_length):
                self.combination[j] = self.combination[j - 1] + 1
        return res
        
    def hasNext(self) -> bool:
        return self.has_next


class CombinationIteratorTest(unittest.TestCase):
    def test_1(self) -> None:
        comb = CombinationIterator("abc", 2)
        self.assertEqual(comb.next(), "ab")
        self.assertTrue(comb.hasNext())
        self.assertEqual(comb.next(), "ac")
        self.assertTrue(comb.hasNext())
        self.assertEqual(comb.next(), "bc")
        self.assertFalse(comb.hasNext())
