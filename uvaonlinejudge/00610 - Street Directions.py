import sys

def read_test_cases():
    while True:
        n, m = (int(i) for i in input().split())
        if n == 0:
            break
        adj_list = [[] for i in range(n)]
        for i in range(m):
            u, v = (int(j) for j in input().split())
            adj_list[v - 1].append(u - 1)
            adj_list[u - 1].append(v - 1)
        yield n, adj_list

def dfs(adj_list, v, depth, edges):
    low = depth[v]
    for u in adj_list[v]:
        if depth[u] == 0:
            edges.append((v, u))
            depth[u] = depth[v] + 1
            u_low = dfs(adj_list, u, depth, edges)
            if u_low > depth[v]:
                edges.append((u, v))
            if u_low < low:
                low = u_low
        elif depth[v] > depth[u] + 1:
            edges.append((v, u))   
            if low > depth[u]:
                low = depth[u]
    return low

def undirect_to_direct(n, adj_list):
    depth, edges = [0] * n, []
    for v in range(n):
        if depth[v] == 0:
            depth[v] = 1
            dfs(adj_list, v, depth, edges)
    return edges

if __name__ == '__main__':
    sys.setrecursionlimit(10000)
    for i, (n, adj_list) in enumerate(read_test_cases(), 1):
        print(i, end='\n\n')
        for v, u in undirect_to_direct(n, adj_list):
            print( v + 1, u + 1)
        print('#')

