from collections import deque

def bfs(adj_list, v, distances):
    queue = deque([v])
    while queue:
        v = queue.popleft()
        for u in adj_list[v]:
            if distances[v] + 1 < distances[u]:
                distances[u] = distances[v] + 1
                queue.append(u)    

if __name__ == '__main__':
    n, m = (int(i) for i in input().split())
    adj_list = [[] for i in range(n)]
    distances = [float('inf')] * n
    distances[0] = 0
    for i in range(m):
        query = [int(j) for j in input().split()]
        if query[0] == 1:
            v = query[1] - 1
            print(distances[v] if distances[v] < float('inf') else -1)
        else:
            v, u = query[1] - 1, query[2] - 1
            adj_list[v].append(u)
            if distances[v] + 1 < distances[u]:
                bfs(adj_list, v, distances)
