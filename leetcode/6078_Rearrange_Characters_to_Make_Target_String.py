from unittest import TestCase

class Solution:
    def rearrangeCharacters(self, s: str, target: str) -> int:
        cnts = [0] * (ord('z') + 1)
        for ch in s:
            cnts[ord(ch)] += 1
        cnt = 0 
        while True:
            for ch in target:
                if cnts[ord(ch)] <1:
                    return cnt
                cnts[ord(ch)] -= 1
            cnt += 1

class RearrangeCharactersTest(TestCase):
    def test_1(self):
        self.assertEqual(Solution().rearrangeCharacters("ilovecodingonleetcode", "code"), 2)

    def test_2(self):
        self.assertEqual(Solution().rearrangeCharacters("abcba", "abc"), 1)

    def test_3(self):
        self.assertEqual(Solution().rearrangeCharacters("abbaccaddaeea", "aaaaa"), 1)                