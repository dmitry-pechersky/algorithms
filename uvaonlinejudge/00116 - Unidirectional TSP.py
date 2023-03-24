import sys
from math import inf

def read_test_case():
    line = sys.stdin.readline().strip()
    if line == '':
        return None
    n, m = (int(i) for i in line.split())
    matrix = [[0] * m for i in range(n)]
    i = 0
    while i < n * m:
        for value in sys.stdin.readline().split():
            matrix[i // m][i % m] = int(value)
            i += 1
    return n, m, matrix

def  minimal_path_dp(n, m, matrix):
    dp = [[inf] * m for i in range(n)]
    for i in range(n):
        dp[i][m - 1] = matrix[i][m - 1]
    for j in range(m - 2, -1, -1):
        for i in range(n):
            for di in (-1, 0, 1):
                k = (i + di) % n
                if dp[k][j + 1] < dp[i][j]:
                    dp[i][j] = dp[k][j + 1]
            dp[i][j] += matrix[i][j]
    i = 0
    for j in range(1, n):
        if dp[j][0] < dp[i][0]:
            i = j
    path = [i]
    for j in range(1, m):
        l = inf
        for di in (-1, 0, 1):
            k = (i + di) % n
            if k < l and dp[k][j] == dp[i][j - 1] - matrix[i][j - 1]:
                l = k
        i = l
        path.append(i)
    return path, dp[path[0]][0]

if __name__ == '__main__':
    for n, m, matrix in iter(read_test_case, None):
        path, cost = minimal_path_dp(n, m, matrix)
        print(*(i + 1 for i in path))
        print(cost)
