def read_test_case():
    n = int(input())
    if n == 0:
        return None
    adj_list = [[] for i in range(n)]
    while True:
        u, v = [int(i) for i in input().split()]
        if u == 0:
            break
        adj_list[u - 1].append(v - 1)
        adj_list[v - 1].append(u - 1)
    return adj_list, n

def is_bipartite(adj_list, n, colors, start):
    stack = [start]
    colors[start] = 0
    while len(stack) > 0:
        u = stack.pop()
        for v in adj_list[u]:
            if colors[v] == -1:
                colors[v] = 1 - colors[u]
                stack.append(v)
            elif colors[v] == colors[u]:
                return False
    return True

if __name__ == '__main__':
    for adj_list, n in iter(read_test_case, None):
        colors = [-1] * n
        res = True
        for i in range(n):
            if colors[i] == -1 and not is_bipartite(adj_list, n, colors, i):
                res = False
                break
        print('YES' if res else 'NO')


        