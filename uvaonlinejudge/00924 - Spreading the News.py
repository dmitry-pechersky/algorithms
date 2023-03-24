from collections import deque

def read_test_case():
    adj_list = [[int(j) for j in input().split()[1:]] for i in range(int(input()))]
    sources = [int(input()) for i in range(int(input()))]
    return adj_list, sources

def bfs(adj_list, v):
    n = len(adj_list)
    depth, cnts, max_i = [-1] * n, [0] * n, 0
    queue, depth[v] = deque([v]), 0
    while len(queue) > 0:
        v = queue.popleft()
        for u in adj_list[v]:
            if depth[u] == -1:
                depth[u] = depth[v] + 1
                cnts[depth[u]] += 1
                if cnts[max_i] < cnts[depth[u]]:
                    max_i = depth[u]
                queue.append(u)
    return cnts[max_i], max_i

if __name__ == '__main__':
    adj_list, sources = read_test_case()
    for v in sources:
        boom_size, boom_day = bfs(adj_list, v)
        if boom_size == 0:
            print(0)
        else:
            print(boom_size, boom_day)
