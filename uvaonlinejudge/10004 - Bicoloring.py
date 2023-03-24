def read_test_case():
    n = int(input())
    if n == 0:
        return None
    l = int(input())
    adj_list = [[] for i in range(n)]
    for i in range(l):
        u, v = [int(j) for j in input().split()]
        adj_list[u].append(v)
        adj_list[v].append(u)
    return adj_list, n

def is_bicolored(adj_list, n):
    colors, stack = [-1] * n, [0]
    colors[0] = 1
    while len(stack) > 0:
        v = stack.pop()
        for u in adj_list[v]:
            if colors[u] == -1:
                colors[u] = 1 - colors[v]
                stack.append(u)
            elif colors[u] == colors[v]:
                return False
    return True

if __name__ == '__main__':
    for adj_list, n in iter(read_test_case, None):
        print('BICOLORABLE.' if is_bicolored(adj_list, n) else 'NOT BICOLORABLE.')
