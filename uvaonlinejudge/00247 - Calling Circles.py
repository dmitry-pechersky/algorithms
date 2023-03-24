def read_test_case():
    n, m = (int(i) for i in input().split())
    if n == 0:
        return None
    dic, names, adj_list = {}, [], []
    for i in range(m):
        source, target = input().split()
        for name in (source, target):
            if name not in dic:
                names.append(name)
                dic[name] = len(names) - 1
                adj_list.append(set())
        adj_list[dic[source]].add(dic[target])
    return names, adj_list

def tarjan_scc(adj_list):
    n = len(adj_list)
    depth, low = [0] * n, [0] * n
    stack, on_stack = [], [False] * n
    components = []
    for v in range(n):
        if depth[v] == 0:
            strong_connect(adj_list, depth, low, stack, on_stack, components, v, 0)        
    return components

def strong_connect(adj_list, depth, low, stack, on_stack, components, v, d):
    d += 1
    depth[v], low[v] = d, d
    stack.append(v)
    on_stack[v] = True
    for u in adj_list[v]:
        if depth[u] == 0:
            strong_connect(adj_list, depth, low, stack, on_stack, components, u, d)
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

if __name__ == '__main__':
    i = 1
    for names, adj_list in iter(read_test_case, None):
        print('Calling circles for data set {}:'.format(i))
        for component in  tarjan_scc(adj_list):
            print(*(names[v] for v in component), sep=', ')
        print()
        i += 1
        

        
