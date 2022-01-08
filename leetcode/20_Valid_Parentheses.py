import unittest

class Solution:
    def isValid(self, s: str) -> bool:
        stack = []
        for ch in s:
            if ch in ("(", "[", "{"):
                stack.append(ch)
            elif len(stack) == 0:
                return False
            else:
                val = stack.pop()
                if not ((ch == ")" and val == "(") or (ch == "]" and val == "[") or (ch == "}" and val == "{")):
                    return False
        return len(stack) == 0

class TestValidParentheses(unittest.TestCase):
    def test_1(self):
        self.assertTrue(Solution().isValid("()"))    

    def test_2(self):
        self.assertTrue(Solution().isValid("()[]{}"))            

    def test_3(self):
        self.assertFalse(Solution().isValid("(]"))                    

    def test_4(self):
        self.assertFalse(Solution().isValid("([)]"))                    

    def test_5(self):
        self.assertTrue(Solution().isValid("{[]}"))                    
