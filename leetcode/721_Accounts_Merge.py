import unittest
from typing import List

class DisjointSet:
    def __init__(self, n: int):
        self.parents = [i for i in range(n)]
        self.ranks = [0] * n
    
    def find(self, x: int) -> int:
        while x != self.parents[x]:
            x = self.parents[x]
        return x

    def union(self, x: int, y: int) -> None:
        root_x = self.find(x)
        root_y = self.find(y)
        if self.ranks[root_x] > self.ranks[root_y]:
            self.parents[root_y] = root_x
        if self.ranks[root_x] < self.ranks[root_y]:
            self.parents[root_x] = root_y
        else:
            self.parents[root_y] = root_x
            self.ranks[root_x] += 1

class Solution:
    def accountsMerge(self, accounts: List[List[str]]) -> List[List[str]]:
        emails, ds = {}, DisjointSet(len(accounts))
        for i in range(len(accounts)):
            for j in range(1, len(accounts[i])):
                email = accounts[i][j]
                if email not in emails:
                    emails[email] = i
                else:
                    ds.union(i, emails[email])
        roots = {}
        for email, idx in emails.items():
            root = ds.find(idx)
            if root not in roots:
                roots[root] = []
            roots[root].append(email)
        new_accounts = []
        for root in roots:
            new_account = [accounts[root][0]]
            new_account.extend(sorted(roots[root]))
            new_accounts.append(new_account)
        return new_accounts

class AccountsMergeTest(unittest.TestCase):
    def test_1(self) -> None:
        self.assertEqual(
            sorted(Solution().accountsMerge([["John","johnsmith@mail.com","john_newyork@mail.com"],["John","johnsmith@mail.com","john00@mail.com"],["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]])),
            sorted([["John","john00@mail.com","john_newyork@mail.com","johnsmith@mail.com"],["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]])
            )    

    def test_2(self) -> None:
        self.assertEqual(
            sorted(Solution().accountsMerge([["Gabe","Gabe0@m.co","Gabe3@m.co","Gabe1@m.co"],["Kevin","Kevin3@m.co","Kevin5@m.co","Kevin0@m.co"],["Ethan","Ethan5@m.co","Ethan4@m.co","Ethan0@m.co"],["Hanzo","Hanzo3@m.co","Hanzo1@m.co","Hanzo0@m.co"],["Fern","Fern5@m.co","Fern1@m.co","Fern0@m.co"]])),
            sorted([["Ethan","Ethan0@m.co","Ethan4@m.co","Ethan5@m.co"],["Gabe","Gabe0@m.co","Gabe1@m.co","Gabe3@m.co"],["Hanzo","Hanzo0@m.co","Hanzo1@m.co","Hanzo3@m.co"],["Kevin","Kevin0@m.co","Kevin3@m.co","Kevin5@m.co"],["Fern","Fern0@m.co","Fern1@m.co","Fern5@m.co"]])
            )    
