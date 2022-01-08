import unittest

class Solution:
    def _match(self, s: str, p: str, i: int, j: int, cache) -> bool:
        s_n, p_n = len(s), len(p)
        if i < s_n and j < p_n and cache[i][j] is not None:
            return cache[i][j]
        if i == s_n and j == p_n:
            return True
        asterisk = j + 1 < p_n and p[j + 1] == '*'
        res = False
        if asterisk:
            res = res or self._match(s, p, i, j + 2, cache)
        if i < s_n and j < p_n:
            if asterisk:
                if s[i] == p[j] or p[j] == '.':
                    res = res or self._match(s, p, i + 1, j, cache)
            else:
                if s[i] == p[j] or p[j] == '.':
                    res = self._match(s, p, i + 1, j + 1, cache) 
        if i < s_n and j < p_n:
            cache[i][j] = res
        return res

    def isMatch(self, s: str, p: str) -> bool:
        cache = [[None] * len(p) for i in range(len(s))]
        return self._match(s, p, 0 , 0, cache)


class TestRegExp(unittest.TestCase):
    def test_1(self):
        self.assertFalse(Solution().isMatch("aa", "a"))

    def test_2(self):
        self.assertTrue(Solution().isMatch("aa", "a*"))

    def test_3(self):
        self.assertTrue(Solution().isMatch("ab", ".*"))

    def test_4(self):
        self.assertTrue(Solution().isMatch("aab", "c*a*b"))

    def test_5(self):
        self.assertFalse(Solution().isMatch("mississippi", "mis*is*p*."))
