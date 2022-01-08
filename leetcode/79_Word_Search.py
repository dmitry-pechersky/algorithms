import unittest
from typing import List

class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        N, M = len(board), len(board[0])
        def rec(board: List[List[str]], used: List[List[bool]], word: str, i: int, j: int, idx: int):
            if idx >= len(word):
                return True
            for (next_i, next_j) in ((i + 1, j), (i, j + 1), (i - 1, j), (i , j - 1)):
                if  0 <= next_i < N and 0 <= next_j < M and word[idx] == board[next_i][next_j] and not used[next_i][next_j]:
                    used[next_i][next_j] = True
                    if rec(board, used, word, next_i, next_j, idx + 1):
                        return True
                    used[next_i][next_j] = False
            return False
            
        used = [[False] * M for i in range(N)]
        for i in range(N):
            for j in range(M):
                if word[0] == board[i][j]:
                    used[i][j] = True
                    if rec(board, used, word, i, j, 1):
                        return True
                    used[i][j] = False
        return False

class TestExist(unittest.TestCase):
    def test_1(self):
        self.assertTrue(Solution().exist([["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "ABCCED"))    

    def test_2(self):
        self.assertTrue(Solution().exist([["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "SEE"))    

    def test_3(self):
        self.assertFalse(Solution().exist([["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "ABCB"))    

    def test_3(self):
        self.assertTrue(Solution().exist([["C","A","A"],["A","A","A"],["B","C","D"]], "AAB"))    
