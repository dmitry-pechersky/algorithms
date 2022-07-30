from typing import List, Dict
from unittest import TestCase
from functools import reduce

class Solution:
    def smallestSufficientTeam(self, req_skills: List[str], people: List[List[str]]) -> List[int]:
        people_n, skills_n = len(people), len(req_skills)
        skills_dic = {skill: i for i, skill in  enumerate(req_skills)}
        people_skills_bw = [reduce( lambda x, y: x | y, ( 1 << skills_dic[skill] for skill in skills), 0) for skills in people]
        for i in range(people_n):
            for j in range(i + 1, people_n):
                if people_skills_bw[i] | people_skills_bw[j] == people_skills_bw[i]:
                    people_skills_bw[j] = 0
        dp: Dict[int, int] = {0: 0}
        for person in range(people_n):
            if people_skills_bw[person] > 0:
                for skills_bw, people_bw in  list(dp.items()):
                    next_skills_bw = skills_bw | people_skills_bw[person]
                    next_people_bw = people_bw | (1 << person)
                    if next_skills_bw not in dp or bin(dp[next_skills_bw]).count('1') > bin(next_people_bw).count('1'):
                        dp[next_skills_bw] = next_people_bw
        people_bw = dp[(1 << skills_n) - 1]        
        return [i for i in range(people_n) if 1 << i & people_bw  != 0]


class SmallestSufficientTeamTest(TestCase):
    
    def test_1(self):
        self.assertEqual([0,2], sorted(Solution().smallestSufficientTeam(["java","nodejs","reactjs"], [["java"],["nodejs"],["nodejs","reactjs"]])))

    def test_2(self):
        self.assertEqual(
            [1,2], 
            sorted(Solution().smallestSufficientTeam(
                ["algorithms","math","java","reactjs","csharp","aws"], 
                [["algorithms","math","java"],["algorithms","math","reactjs"],["java","csharp","aws"],["reactjs","csharp"],["csharp","math"],["aws","java"]])
            ))

