def dfs(adj_list, v, visited, weights):
    stack = [v]
    visited[v] = True
    cost = 0
    while stack:
        v = stack.pop()
        cost += weights[v]
        for u in adj_list[v]:
            if not visited[u]:
                stack.append(u)
                visited[u] = True
    return cost

def connected_components(adj_list, n, weights):
    visited = [False] * n
    max_cost = 0
    for v in range(n):
        if not visited[v]:
            max_cost = max(max_cost, dfs(adj_list, v, visited, weights))
    return max_cost

if __name__ == '__main__':
    for t in range(int(input())):
        n, m = map(int, input().split())
        adj_list = [[] for i in range(n)]
        for i in range(m):
            u, v = (int(j) - 1 for j in input().split())
            adj_list[u].append(v)
            adj_list[v].append(u)
        weights = [int(i) for i in input().split()]
        print(connected_components(adj_list, n, weights))
