def read_test_case():
    n, m = (int(i) for i in input().split())
    if n == 0:
        return None
    adj_list = [[] for i in range(n)]
    for i in range(m):
        u, v = (int(j) for j in input().split())
        adj_list[u].append(v)
        adj_list[v].append(u)
    return n, adj_list

def longest_path_recursive(n, adj_list, v, visited):
    max_length = 0
    for u in adj_list[v]:
        if visited[v][u] != 1:
            visited[v][u], visited[u][v] = 1, 1
            max_length = max(max_length, 1 + longest_path_recursive(n, adj_list, u, visited))
            visited[v][u], visited[u][v] = 0, 0
    return max_length

def longest_path(n, adj_list):
    max_length = 0
    visited = [[0] * n for i in range(n)]
    for v in range(n):
        max_length = max(max_length, longest_path_recursive(n, adj_list, v, visited))
    return max_length

if __name__ == '__main__':
    for n, adj_list in  iter(read_test_case, None):
        print(longest_path(n, adj_list))
