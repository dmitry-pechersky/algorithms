import heapq

def dijkstra(n, adj_list, starts):
    distances = [None] * n
    heap = []
    for u in starts:
        heapq.heappush(heap, (0, u))
    while len(heap) > 0:
        cost, u = heapq.heappop(heap)
        if distances[u] is None:
            distances[u] = cost
            for v, weigth in adj_list[u]:
                heapq.heappush(heap, (cost + weigth, v))
    return distances

if __name__ == '__main__':
    n, m = (int(i) for i in input().split())
    adj_list = [[] for i in range(n)]
    for j in range(m):
        u, v, weigth = (int(i) for i in input().split())
        adj_list[u - 1].append((v - 1, weigth))
        adj_list[v - 1].append((u - 1, weigth))
    _ = int(input())
    starts = [int(i) - 1 for i in input().split()]
    print(*dijkstra(n, adj_list, starts), sep='\n')
