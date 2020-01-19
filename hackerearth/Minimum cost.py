from heapq import heappush, heappop

def successors(node, n, permutaion):
    if node > 0:
        yield (node - 1, 1)
    if node < n - 1:
        yield (node + 1, 1)
    yield (permutaion[node], 0)

def dijkstra(n, permutaion):
    heap, costs = [(0,0)], [None for i in range(n)]
    while len(heap) > 0:
        cost, node = heappop(heap)
        if node == n - 1:
            return cost
        if costs[node] is None:
            costs[node] = cost
            for successor, weight in successors(node, n, permutaion):
                heappush(heap, (cost + weight, successor))

if __name__ == '__main__':
    for i in range(int(input())):
        n, permutaion  = int(input()), [int(j) - 1 for j in input().split()]
        print(dijkstra(n, permutaion))
        
