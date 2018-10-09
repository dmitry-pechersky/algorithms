def read_test_case():
    n, k = (int(i) for i in input().split())
    adj_list = [[] for i in range(n)]
    for i in range(k):
        u, v = (int(i) for i in input().split())
        adj_list[u].append(v)
        adj_list[v].append(u)
    return n, adj_list

def dfs(adj_list, visited, v):
    stack, visited[v], cnt = [v], True, 1
    while len(stack) > 0:
        v = stack.pop()
        for u in adj_list[v]:
            if not visited[u]:
                stack.append(u)
                visited[u] = True
                cnt += 1
    return cnt

def connected_components(n, adj_list):
    visited, total_cnt, total_pairs = [False] * n, 0, 0
    for v in range(n):
        if not visited[v]:
            cnt = dfs(adj_list, visited, v)
            total_pairs += total_cnt * cnt
            total_cnt += cnt
    return total_pairs

if __name__ == '__main__':
    print(connected_components(*read_test_case()))
