import heapq

def read_test_case():
    n, m, source, target = (int(i) for i in input().split())
    adj_list = [[] for i in range(n)]
    for i in range(m):
        u, v, weight = (int(i) for i in input().split())
        adj_list[u].append((v, weight))
        adj_list[v].append((u, weight))
    return n, source, target, adj_list

def dijkstra(n, source, target, adj_list):
    queue, costs = [], [None] * n
    costs[source] = 0
    heapq.heappush(queue, (0, source))
    while len(queue) > 0:
        cost, v = heapq.heappop(queue)
        if v == target:
            break
        if cost > costs[v]:
            continue
        for u, weight in adj_list[v]:
            if costs[u] is None or costs[u] > cost + weight:
                costs[u] = cost + weight
                heapq.heappush(queue, (cost + weight, u))

    return costs[target]

if __name__ == '__main__':
    for i in range(1, int(input()) + 1):
        cost = dijkstra(*read_test_case())
        print("Case #{}: {}".format(i, 'unreachable' if cost is None else cost))