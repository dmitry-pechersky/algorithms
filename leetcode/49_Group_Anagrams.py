import unittest
from typing import List

class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        dic = dict()
        for string in strs:
            key = "".join(sorted(string))
            if key not in dic:
                dic[key] = []
            dic[key].append(string)
        return list(dic.values())

class TestGroupAnagrams(unittest.TestCase):
    def sorted_list_of_lists(self, lst: List[List[str]]):
        return sorted([sorted(i) for i in lst])

    def test_1(self):
        self.assertEqual(
            self.sorted_list_of_lists(Solution().groupAnagrams(["eat","tea","tan","ate","nat","bat"])), 
            self.sorted_list_of_lists([["bat"],["nat","tan"],["ate","eat","tea"]])
        )        

    def test_2(self):
        self.assertEqual(Solution().groupAnagrams([""]), [[""]])        

    def test_3(self):
        self.assertEqual(Solution().groupAnagrams(["a"]), [["a"]])        
