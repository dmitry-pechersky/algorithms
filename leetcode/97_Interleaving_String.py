import unittest

class Solution:
    def isInterleave(self, s1: str, s2: str, s3: str) -> bool:
        if len(s1) + len(s2) != len(s3):
            return False
        dp = [False] * (len(s2) + 1)
        dp[0] = True
        for j in range(1, len(s2) + 1):
            dp[j] = s2[j - 1] == s3[j - 1]
            if not dp[j]:
                break
        for i in range(1, len(s1) + 1):
            for j in range(len(s2) + 1):
                dp[j] = (j > 0 and dp[j - 1] and s2[j - 1] == s3[ i + j - 1]) or (dp[j] and s1[i - 1] == s3[ i + j - 1])
        return dp[len(s2)]

class IsInterleaveTest(unittest.TestCase):
    def test_1(self) -> None:
        self.assertTrue(Solution().isInterleave("aabcc", "dbbca", "aadbbcbcac"))

    def test_2(self) -> None:
        self.assertFalse(Solution().isInterleave("aabcc", "dbbca", "aadbbbaccc"))

    def test_3(self) -> None:
        self.assertTrue(Solution().isInterleave("", "", ""))

