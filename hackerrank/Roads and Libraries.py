def read_test_cases():
    for t in range(int(input())):
        n, m , cost_lib, cost_road = (int(i) for i in input().split())
        adj_list = [[] for i in range(n)]
        for i in range(m):
            v, u = (int(i) for i in input().split())
            adj_list[v - 1].append(u - 1)
            adj_list[u - 1].append(v - 1)
        yield n, cost_lib, cost_road, adj_list

def dfs(adj_list, visited, v):
    stack, visited[v] = [v], True
    while len(stack) > 0:
        v = stack.pop()
        for u in adj_list[v]:
            if not visited[u]:
                stack.append(u)
                visited[u] = True

def connected_components(n, adj_list):
    visited, cnt = [False] * n, 0
    for v in range(n):
        if not visited[v]:
            dfs(adj_list, visited, v)
            cnt += 1
    return cnt

if __name__ == '__main__':
    for n, cost_lib, cost_road, adj_list in read_test_cases():
        if cost_lib < cost_road:
            print(cost_lib * n)
        else:
            cnt = connected_components(n, adj_list)
            print(cnt * cost_lib + (n - cnt) * cost_road)