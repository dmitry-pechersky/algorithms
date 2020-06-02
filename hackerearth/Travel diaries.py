import sys 
from collections import deque

def read_int():
    while True:
        for line in sys.stdin.readlines():
            for val in line.split():
                yield int(val)

def bfs(n, m, matrix):
    max_distance = 0
    queue = deque([])
    for i in range(n):
        for j in range(m):
            if matrix[i][j] == 2:
                queue.append((0, i, j))
    while queue:
        dist, i, j = queue.popleft()
        if dist > max_distance:
            max_distance = dist
        for delta_i, delta_j in ((0, 1), (1, 0), (0, -1), (-1, 0)):
            if  0 <= i + delta_i < n and 0 <= j + delta_j < m and  matrix[i + delta_i][j + delta_j] == 1:
                queue.append((dist + 1, i + delta_i, j + delta_j))
                matrix[i + delta_i][j + delta_j] = 0
    for i in range(n):
        for j in range(m):
            if matrix[i][j] == 1:
                return -1
    return max_distance

if __name__ == '__main__':
    gen = read_int()
    n, m = next(gen), next(gen)
    matrix = [[next(gen) for j in range(m)] for i in range(n)]
    print(bfs(n, m, matrix))
    