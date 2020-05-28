def dfs(adj_list, n):
    parents = [None] * n
    cnts = [1] * n
    stack = [0]
    parents[0] = -1
    order = []
    while stack:
        v = stack.pop()
        order.append(v)
        for u in adj_list[v]:
            if parents[u] is None:
                stack.append(u)
                parents[u] = v
    while len(order) > 1:
        v = order.pop()
        cnts[parents[v]] += cnts[v]
    return cnts

if __name__ == '__main__':
    n, q = (int(i) for i in  input().split())
    adj_list = [[] for i in range(n)]
    edges = []
    for i in range(n - 1):
        u, v = (int(j) - 1 for j in  input().split())
        adj_list[u].append(v)
        adj_list[v].append(u)
        edges.append((u, v))
    cnts = dfs(adj_list, n)
    for i in range(q):
        u, v = edges[int(input()) - 1]
        cnt = cnts[u] if cnts[u] < cnts[v] else cnts[v]
        print(cnt * (n - cnt))
