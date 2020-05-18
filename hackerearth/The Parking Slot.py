import heapq 

def dijkstra(adj_list, n, capacities, k):
    heap = [(0, 0)]
    distances = [None] * n
    nodes_order = []
    while heap and k > 0:
        distance, u = heapq.heappop(heap)
        if distances[u] is None:
            distances[u] = distance
            nodes_order.append(u)
            k -= capacities[u]
            for v, weight in adj_list[u]:
                if distances[v] is None:
                    heapq.heappush(heap, (distance + weight, v))
    return distances, nodes_order

def park(distances, parking_order, capacities, k, f):
    costs = [-1] * k
    car = 0
    for parking in parking_order:
        for j in range(capacities[parking]):
            if car < k:
                costs[car] = f + distances[parking]
                car += 1
            else:
                break
    return costs

if __name__ == '__main__':
    n, m, f = (int(i) for i in input().split())
    adj_list = [[] for i in range(n)]
    capacities = [int(i) for i in input().split()]
    for i in range(m):
        u, v, weight = (int(i) for i in input().split())
        adj_list[u - 1].append((v - 1, weight))
        adj_list[v - 1].append((u - 1, weight))
    k = int(input())
    distances, parking_order = dijkstra(adj_list, n, capacities, k)
    print(*park(distances, parking_order, capacities, k, f))

