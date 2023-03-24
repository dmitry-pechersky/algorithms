from collections import deque
import math

def read_test_case():
    n = int(input())
    if n == 0:
        return None
    adj_matrix = [[0] * n for i in range(n)]
    source, sink, m = (int(i) for i in input().split())
    for i in range(m):
        v, u, weight = (int(i) for i in input().split())
        adj_matrix[v - 1][u  - 1] += weight
        adj_matrix[u - 1][v  - 1] += weight
    return n, source - 1, sink - 1, adj_matrix

def bfs(n, adj_matrix, start, end):
    parent = [None] * n 
    parent[start] = start
    queue = deque([start])
    while len(queue) > 0:
        v = queue.popleft()
        for u in range(n):
            if adj_matrix[v][u] > 0 and parent[u] is None:
                parent[u] = v
                if u == end:
                    return parent
                queue.append(u)
    return parent

def edmonds_karp(n, adj_matrix, source, sink):
    full_flow = 0
    while True:
        parent = bfs(n, adj_matrix, source, sink)
        if parent[sink] is None:
            break
        v, flow = sink, math.inf
        while v != source:
            flow = min(flow, adj_matrix[parent[v]][v])
            v = parent[v]
        v = sink
        while v!= source:
            adj_matrix[parent[v]][v] -= flow
            adj_matrix[v][parent[v]] += flow
            v = parent[v]
        full_flow += flow
    return full_flow

if __name__ == '__main__':
    for i, (n, source, sink, adj_matrix) in enumerate(iter(read_test_case, None), 1):
        print('Network {}'.format(i))
        print('The bandwidth is {}.\n'.format(edmonds_karp(n, adj_matrix, source, sink)))
