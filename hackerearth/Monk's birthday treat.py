def dfs(adj_list, n, v):
    visited = [False] * n
    stack, visited[v] = [v], True
    cnt = 0
    while stack:
        v = stack.pop()
        cnt += 1
        for u in adj_list[v]:
            if not visited[u]:
                stack.append(u)
                visited[u] = True
    return cnt

if __name__ == '__main__':
    n, m = map(int, input().split())
    adj_list =[[] for i in range(n)]
    for i in range(m):
        u, v = map(int, input().split())
        adj_list[u - 1].append(v - 1)
    min_cnt = n
    for v in range(n):
        min_cnt = min(min_cnt, dfs(adj_list, n, v))
    print(min_cnt)
