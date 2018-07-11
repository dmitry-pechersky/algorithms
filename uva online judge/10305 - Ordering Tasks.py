def read_test_case():
    n, m = (int(i) for i in input().split())
    if n == 0:
        return None
    adj_list = [[] for i in range(n)]
    for i in range(m):
        v, u = (int(i) - 1 for i in input().split())
        adj_list[v].append(u)
    return n, adj_list

def dfs(adj_list, v, visited, nodes):
    visited[v] = True
    for u in adj_list[v]:
        if not visited[u]:
            dfs(adj_list, u, visited, nodes)
    nodes.append(v)

def toposort(n, adj_list):
    nodes = []
    visited = [False] * n
    for v in range(n):
        if not visited[v]:
            dfs(adj_list, v, visited, nodes)
    nodes.reverse()
    return nodes

if __name__ == '__main__':
    for n, adj_list in iter(read_test_case, None):
        print(*(i + 1 for i in toposort(n, adj_list)))
