def read_test_case():
    n, e = [int(i) for i in input().split()]
    adj_list = [[] for i in range(n)]
    for i in range(e):
        u, v = [int(i) for i in input().split()]
        adj_list[u].append(v)
        adj_list[v].append(u)
    return adj_list, n

def bicolor(adj_list, colors, n, start):
    colors[start], cnts = 0, [1, 0]
    stack = [start]
    while len(stack) > 0:
        u = stack.pop()
        for v in adj_list[u]:
            if colors[v] == -1:
                colors[v] = 1 - colors[u]
                cnts[colors[v]] += 1
                stack.append(v)
            elif colors[v] == colors[u]:                
                return -1
    return min(cnts)

if __name__ == '__main__':
    for i in range(int(input())):
        adj_list, n = read_test_case()
        cnt, colors = 0, [-1] * n
        for i in range(n):
            if colors[i] == -1:                
                min_cnt = bicolor(adj_list, colors, n, i)
                if min_cnt == -1:
                    cnt = -1 
                    break
                cnt += min_cnt if min_cnt > 0 else 1
        print(cnt)
