import unittest

class Solution:
    def simplifyPath(self, path: str) -> str:
        stack = []
        for token in path.split("/"):
            if token not in ("", "."):
                if token != '..':
                    stack.append(token)
                elif len(stack) > 0:
                    stack.pop()
        return "/" + "/".join(stack)

class TestMinPathSum(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().simplifyPath("/a/./b/../../c/"), "/c")

    def test_2(self):
        self.assertEqual(Solution().simplifyPath("/home//foo/"), "/home/foo")

    def test_3(self):
        self.assertEqual(Solution().simplifyPath("/../"), "/")

    def test_4(self):
        self.assertEqual(Solution().simplifyPath("/home/"), "/home")
