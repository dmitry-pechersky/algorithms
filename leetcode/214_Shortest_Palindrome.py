import unittest

def z_function(s: str):
    n = len(s)
    z = [0] * n
    l, r = 0, 0
    for i in range(1, n):
        if i <= r:
            z[i] = z[i - l] if z[i - l] < r - i + 1 else r - i + 1
        while i + z[i] < n and s[z[i]] == s[i + z[i]]:
            z[i] += 1
        if i + z[i] - 1 > r:
            l, r = i, i + z[i] - 1
    return z

class Solution:
    def shortestPalindrome(self, s: str) -> str:
        n = len(s)
        z = z_function(s + '$' + s[::-1])
        for i in range(n + 1, n + n + 1):
            if z[i] == n + n + 1 - i:
                return s[n - 1: z[i] - 1: -1] + s
        return ''

class TestZFunction(unittest.TestCase):
    def test_a(self):
        self.assertEqual(z_function('a'), [0])    

    def test_aaaaa(self):
        self.assertEqual(z_function('aaaaa'), [0, 4 , 3, 2 , 1])    

    def test_abaabaaba(self):
        self.assertEqual(z_function('abaabaaba'), [0, 0, 1, 6, 0, 1, 3, 0, 1])    

class TestShortestPalindrome(unittest.TestCase):
    def test_aacecaaa(self):
        self.assertEqual(Solution().shortestPalindrome('aacecaaa'), 'aaacecaaa')    

    def test_abcd(self):
        self.assertEqual(Solution().shortestPalindrome('abcd'), 'dcbabcd')    

    def test_a(self):
        self.assertEqual(Solution().shortestPalindrome('a'), 'a')    

    def test_empty(self):
        self.assertEqual(Solution().shortestPalindrome(''), '')    

if __name__ == '__main__':
    pass
    
