def read_test_case():
    m = int(input())
    if m == 0:
        return None
    adj_list = []
    dic, words = {}, []
    for i in range(m):
        letters = input().split()
        for letter in letters:
            if letter not in dic:
                words.append(letter)
                dic[letter] = len(words) - 1
                adj_list.append([])
        adj_list[dic[letters[-1]]].extend(dic[letter] for letter in letters[:-1])
    return len(words), adj_list, words

def strong_connect(adj_list, v, depth, low, d, stack, on_stack, components):
    depth[v], low[v] = d + 1, d + 1
    stack.append(v)
    on_stack[v] = True
    for u in adj_list[v]:
        if depth[u] == 0:
            strong_connect(adj_list, u, depth, low, d + 1, stack, on_stack, components)
        if on_stack[u]:
            low[v] = min(low[v], low[u])
    if depth[v] == low[v]:
        component = []
        while True:
            u = stack.pop()
            on_stack[u] = False
            component.append(u)
            if v == u:
                break
        components.append(component)

def tarjan_scc(n, adj_list):
    low, depth = [0] * n, [0] * n
    stack, on_stack = [], [False] * n
    components = []
    for v in range(n):
        if depth[v] == 0:
            strong_connect(adj_list, v, depth, low, 0, stack, on_stack, components)
    return components

if __name__ == '__main__':
    for i, (n, adj_list, words) in enumerate(iter(read_test_case, None)):
        if i > 0:
            print()
        components = tarjan_scc(n, adj_list)
        for row in sorted(sorted(words[i] for i in component) for component in components):
            print(*row)
