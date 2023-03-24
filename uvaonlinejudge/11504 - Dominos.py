import sys

def read_test_cases():
    for i in range(int(input())):
        n, m = (int(j) for j in input().split())
        adj_list = [[] for j in range(n)]
        for j in range(m):
            u, v = (int(l) for l in input().split())
            adj_list[u - 1].append(v - 1)
        yield n, adj_list

def tarjan(adj_list, v, depth, low, stack, on_stack, d):
    d += 1
    depth[v], low[v], on_stack[v] = d, d, True
    stack.append(v)
    for u in adj_list[v]:
        if depth[u] is None:
            d = tarjan(adj_list, u, depth, low, stack, on_stack, d)
        if on_stack[u] and low[u] < low[v]:
            low[v] = low[u]
    if depth[v] == low[v]:
        while True:
            u = stack.pop()
            on_stack[u] = False
            if v == u:
                break
            low[u] = depth[v]
    return d

def scc_tarjan(n, adj_list):
    d, depth, low, stack, on_stack = -1, [None] * n, [0] * n, [], [False] * n
    for v in range(n):
        if depth[v] is None:
            d = tarjan(adj_list, v, depth, low, stack, on_stack, d)
    cnt, components, in_edges = 0, [False] * n, [False] * n
    for v in range(n):
        if not components[low[v]]:
            cnt += 1
            components[low[v]] = True
        for u in adj_list[v]:
            if not in_edges[low[u]] and low[u] != low[v]:
                cnt -= 1
                in_edges[low[u]] = True
    return cnt

if __name__ == '__main__':
    sys.setrecursionlimit(1000000)
    for n, adj_list in read_test_cases():
        print(scc_tarjan(n, adj_list))
