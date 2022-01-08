import unittest

class Solution:
    def multiply(self, num1: str, num2: str) -> str:
        n1, n2 = len(num1), len(num2)
        num1, num2, num3 = [int(i) for i in reversed(num1)], [int(i) for i in  reversed(num2)], [0] * (n1 + n2)
        for i in range(n1):
            trans = 0
            for j in range(n2):
                k = i + j
                tmp = num3[k] + num1[i] * num2[j] + trans
                num3[k], trans = tmp % 10, tmp // 10
            if trans > 0:
                num3[i + n2] += trans 
        while len(num3) > 1 and num3[-1] == 0:
            num3.pop()
        return "".join(str(i) for i in reversed(num3))

class TestMultiply(unittest.TestCase):
    def test_1(self):
        self.assertEqual(Solution().multiply("2", "3"), "6")

    def test_2(self):
        self.assertEqual(Solution().multiply("123", "456"), "56088")

    def test_3(self):
        self.assertEqual(Solution().multiply("9133", "0"), "0")
