import sys

def read_test_case():
    line = sys.stdin.readline().strip()
    if line == '':
        return None
    n = int(line)
    adj_list = [None] * n
    for i in range(n):
        line = sys.stdin.readline().split()
        adj_list[int(line[0])] = [int(j) for j in line[2:]]
    sys.stdin.readline()
    return adj_list, n
    
def articulation_bridges(adj_list, n, visited, parents, depth, low, bridges, v, d):
    visited[v] = True
    depth[v], low[v] = d, d
    for u in adj_list[v]:
        if not visited[u]:
            parents[u] = v
            articulation_bridges(adj_list, n, visited, parents, depth, low, bridges, u, d + 1)
            if low[u] > depth[v]:
                bridges.append((min(v,u), max(v,u)))
            low[v] = min(low[v], low[u])
        elif parents[v] != u:
            low[v] = min(low[v], depth[u])

if __name__ == '__main__':
    for adj_list, n in iter(read_test_case, None):
        visited, parents = [False] *n, [None] * n
        low, depth = [0] * n, [0] * n
        bridges = []
        for v in range(n):
            if not visited[v]:
                articulation_bridges(adj_list, n, visited, parents, depth, low, bridges, v, 1)
        print(len(bridges), 'critical links')
        bridges.sort()
        for v, u in bridges:
            print(v, '-', u)
        print()