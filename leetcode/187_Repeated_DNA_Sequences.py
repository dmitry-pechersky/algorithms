from typing import List, Dict
from unittest import TestCase

class Solution:
    def findRepeatedDnaSequences(self, s: str) -> List[str]:
        n = len(s)
        mask = 0b11111111111111111111
        cur_hash = 0
        dic: Dict[int, int] = {}
        res: List[str] = []
        for i in range(n):
            ch = 0 if s[i] == 'A' else (1 if s[i] == 'C' else (2 if s[i] =='G' else 3))
            cur_hash = (cur_hash << 2 | ch) & mask
            if i >= 9:
                dic[cur_hash] = dic.get(cur_hash, 0) + 1
                if dic[cur_hash] == 2:
                    res.append(s[i - 9:i + 1])
        return res

class FindRepeatedDnaSequencesTest(TestCase):
    def test_1(self) -> None:
        self.assertEqual(
            ["AAAAACCCCC","CCCCCAAAAA"],
            sorted(Solution().findRepeatedDnaSequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"))
            )

    def test_2(self) -> None:
        self.assertEqual(
            ["AAAAAAAAAA"],
            Solution().findRepeatedDnaSequences("AAAAAAAAAAAAA")
            )
