import sys

def read_test_case():
    n = ord(sys.stdin.readline().strip()) - 65 + 1
    adj_list = [[] for i in range(n)]
    while True:
        line = sys.stdin.readline().strip()
        if len(line) != 2:
            break
        v, u = (ord(i) - 65 for i in line)
        adj_list[v].append(u)
        adj_list[u].append(v)
    return n, adj_list

def dfs(adj_list, idx, v, i):
    idx[v] = i
    for u in adj_list[v]:
        if idx[u] is None:
            dfs(adj_list, idx, u, i)

def connected_components(n, adj_list):
    idx = [None] * n
    i = 0
    for v in range(n):
        if idx[v] is None:
            dfs(adj_list, idx, v, i)
            i +=  1
    return i 
        
if __name__ == '__main__':
    t = int(sys.stdin.readline())
    sys.stdin.readline()
    for i in range(t):
        if i > 0:
            print()
        print(connected_components(*read_test_case()))
