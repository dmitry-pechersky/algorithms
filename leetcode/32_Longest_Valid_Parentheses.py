import unittest

class Solution:
    def longestValidParentheses(self, s: str) -> int:
        n = len(s)
        stack, valids = [], [False] * n
        for i in range(n):
            if s[i] == '(':
                stack.append(i)
            elif len(stack) > 0:
                valids[stack.pop()] = True
                valids[i] = True
        cnt, max_cnt = 0, 0
        for valid in valids:
            if valid:
                cnt += 1
                max_cnt = cnt if cnt > max_cnt else max_cnt
            else:
                cnt = 0
        return max_cnt

class TestLongestValidParentheses(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().longestValidParentheses("(()"), 2)    

    def test_2(self):
        self.assertEqual(Solution().longestValidParentheses(")()())"), 4)    

    def test_3(self):
        self.assertEqual(Solution().longestValidParentheses(""), 0)    

    def test_4(self):
        self.assertEqual(Solution().longestValidParentheses("()(()"), 2)    
