from heapq import heappush, heappop

def read_test_case():
    n, e = (int(i) for i in input().split())
    adj_list = [[] for i in range(n)]
    for i in range(e):
        u, v, cost = (int(i) for i in input().split())
        adj_list[u - 1].append((v - 1, cost))
        adj_list[v - 1].append((u - 1, cost))
    return n, adj_list

def dijkstra(n, adj_list):
    heap, expanded = [], [False] * n
    heappush(heap, (0, 0))
    while len(heap) > 0:
        cum_cost, v = heappop(heap)
        if v == n -1:
            return cum_cost
        expanded[v] = True
        for u, cost in adj_list[v]:
            if not expanded[u]:
                heappush(heap, (cum_cost + (cost - cum_cost if cost > cum_cost else 0), u))
    return -1

if __name__ == '__main__':
    cost = dijkstra(*read_test_case())
    print(cost if cost >= 0 else 'NO PATH EXISTS')
