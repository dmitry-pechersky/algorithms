def read_test_case():
    n = int(input())
    if n == 0:
        return None
    adj_list = [None for i in range(n)]
    words, dic = [], {}
    for i in range(n):
        line = input().split()
        for word in line:
            if word not in dic:
                words.append(word)
                dic[word] = len(words) - 1
        adj_list[dic[line[0]]] = [dic[word] for word in line[1:]]
    return n, adj_list, words

def tarjan_scc(n, adj_list):
    depth, low = [0] * n, [0] * n
    stack, on_stack = [], [False] * n
    for v in range(n):
        if depth[v] == 0:
            tarjan_strong_connect(adj_list, depth, low, stack, on_stack, v, 0)  
    return depth, low  

def dfs(adj_list, visited, v):
    for u in adj_list[v]:
        if not visited[u]:
            visited[u] = True
            dfs(adj_list, visited, u)

def tarjan_strong_connect(adj_list, depth, low, stack, on_stack, v, d):
    d += 1
    depth[v], low[v], on_stack[v] = d, d, True
    stack.append(v)
    for u in adj_list[v]:
        if depth[u] == 0:
            tarjan_strong_connect(adj_list, depth, low, stack, on_stack, u, d)
        if on_stack[u]:
            low[v] = min(low[v], low[u])
    if depth[v] == low[v]:
        while True:
            u = stack.pop()
            on_stack[u] = False
            if v == u:
                break

if __name__ == '__main__':
    for n, adj_list, words in iter(read_test_case, None):
        depth, low = tarjan_scc(n, adj_list)
        visited = [False] * n
        for v in range(n):
            if depth[v] != low[v] and not visited[v]:
                dfs(adj_list, visited, v)        
        print(sum(1 for i in visited if i))
        print(*sorted(words[v] for v in range(n) if visited[v]))
