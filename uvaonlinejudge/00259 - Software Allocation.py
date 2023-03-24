import sys
import math
from collections import deque

n, source, sink = 38, 0, 37

def read_test_case():
    end_of_cases = True
    adj_matrix = [[0] * n for i in range(n)]
    while True:
        line = sys.stdin.readline().strip()
        if line == '':
            break
        end_of_cases = False
        v = ord(line[0]) - 65 + 1
        adj_matrix[source][v] = int(line[1]) 
        for u in (27 + int(i) for i in line[3:-1]):
            adj_matrix[v][u] = math.inf
            adj_matrix[u][sink] = 1
    return None if end_of_cases else adj_matrix

def bfs(adj_matrix, n, start, end):
    queue = deque([start])
    parent = [None] * n
    parent[start] = start
    while len(queue) > 0:
        v = queue.popleft()
        for u in range(n):
            if adj_matrix[v][u] > 0 and parent[u] is None:
                parent[u] = v
                if u == end:
                    return parent
                queue.append(u)
    return parent

def edmonds_karp(adj_matrix, n, source, sink):
    full_flow = 0
    while True:
        parent = bfs(adj_matrix, n, source, sink)
        if parent[sink] is None:
            break
        v, flow = sink, math.inf
        while v != source:
            flow = min(flow, adj_matrix[parent[v]][v])
            v = parent[v]
        v = sink
        while v != source:
            adj_matrix[parent[v]][v] -= flow
            adj_matrix[v][parent[v]] += flow
            v = parent[v]        
        full_flow += flow
    return full_flow

if __name__ == '__main__':
    for adj_matrix in iter(read_test_case, None):
        flow = edmonds_karp(adj_matrix, n, source, sink)
        if sum(adj_matrix[source]) == 0:
            output = ['_'] * 10
            for i in range(10):
                u = 27 + i 
                for v in range(1, 27):
                    if adj_matrix[u][v] > 0:
                        output[i] = chr(v - 1 + 65)
                        break
            print(*output, sep='')
        else:
            print('!')
