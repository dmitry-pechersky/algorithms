def read_test_case():
    n, m = (int(i) for i in input().split())
    adj_list = [[] for i in range(n)]
    for i in range(m):
        v, u = (int(i) - 1 for i in input().split())
        adj_list[u].append(v)
        adj_list[v].append(u)
    return n, m, adj_list

def dfs(adj_list, visited, v):
    stack, cnt, visited[v] = [v], 1, True
    while len(stack) > 0:
        v = stack.pop()
        for u in adj_list[v]:
            if not visited[u]:
                visited[u] = True
                stack.append(u)
                cnt += 1
    return cnt

def connected_components(n, adj_list):
    visited = [False] * n
    sizes = []
    for v in range(n):
        if not visited[v]:
            sizes.append(dfs(adj_list, visited, v))
    return sizes

if __name__ == '__main__':
    for t in range(int(input())):
        n, m, adj_list = read_test_case()
        total, friends, friendships = 0, 0, 0
        for size in sorted(connected_components(n, adj_list), reverse=True):
            for i in range(2, size + 1):
                friends += i * (i - 1) - (i - 1) * (i - 2)
                total += friends
                friendships += 1
        total += (m - friendships) * friends
        print(total)