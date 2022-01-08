import unittest

class Solution:
    def isMatch(self, s: str, p: str) -> bool:
        s_n, p_n = len(s), len(p)
        stack, dp = [(0, 0)], set()
        while len(stack) > 0:
            i, j = stack.pop()
            if (i, j) not in dp:
                dp.add((i,j))
                if i < s_n and j < p_n:
                    if s[i] == p[j] or p[j] == '?':
                        stack.append((i + 1, j + 1))
                    elif p[j] == '*':
                        stack.append((i + 1, j + 1))
                        stack.append((i + 1, j))
                        stack.append((i, j + 1))
                elif i == s_n and j < p_n and p[j] == '*':
                    stack.append((i, j + 1))
                elif i == s_n and j == p_n:
                    return True
        return False

class TestIsMatch(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().isMatch("aa", "a"), False)

    def test_2(self):
        self.assertEqual(Solution().isMatch("aa", "*"), True)

    def test_3(self):
        self.assertEqual(Solution().isMatch("cb", "?a"), False)

    def test_4(self):
        self.assertEqual(Solution().isMatch("adceb", "*a*b"), True)

    def test_5(self):
        self.assertEqual(Solution().isMatch("acdcb", "a*c?b"), False)

    def test_5(self):
        self.assertEqual(Solution().isMatch("", "**"), True)
