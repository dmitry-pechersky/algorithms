def read_test_case():
    input()
    n = int(input())
    adj_list = [[] for i in range(n)]
    for u in range(n):
        for v in [int(i) - 1 for i in input().split()[1:]]:
            if v < n:
                adj_list[u].append(v)
                adj_list[v].append(u)
    return adj_list, n

def bicolor(adj_list, colors, start):
    stack = [start]
    colors[start] = 0
    is_bipartite = True
    color_cnts = [1, 0]
    while len(stack) > 0:
        u = stack.pop()
        for v in adj_list[u]:
            if colors[v] == -1:
                colors[v] = 1 - colors[u]
                color_cnts[colors[v]] += 1
                stack.append(v)
            elif colors[v] == colors[u]:
                is_bipartite = False
    return is_bipartite, color_cnts

if __name__ == '__main__':
    for i in range(int(input())):
        adj_list, n = read_test_case()
        colors = [-1] * n
        cnt = 0
        for start in range(n):
            if colors[start] == -1:
                is_bipartite, color_cnts = bicolor(adj_list, colors, start)
                if is_bipartite:
                    cnt += max(color_cnts)
        print(cnt)

        

