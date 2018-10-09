def read_test_case():
    n, k = (int(i) for i in input().split())
    adj_list = [[] for i in range(n)]
    for i in range(k):
        v, u = (int(i) for i in input().split())
        adj_list[v - 1].append(u - 1)
        adj_list[u - 1].append(v - 1)
    return n, adj_list

def dfs_rec(n, adj_list, v, visited):
    total_cuts, total_nodes = 0, 1
    visited[v] = True
    for u in adj_list[v]:
        if not visited[u]:
            cuts, nodes = dfs_rec(n, adj_list, u, visited)
            total_cuts += cuts
            total_nodes += nodes
    if total_nodes % 2 == 0:
         total_cuts += 1
    return total_cuts, total_nodes

if __name__ == '__main__':
    n, adj_list = read_test_case()
    visited = [False] * n
    print(dfs_rec(n, adj_list, 0, visited)[0] - 1)
    