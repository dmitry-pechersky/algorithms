def bellman_ford(n, edges):
    costs = [float('-inf')] * n
    costs[0] = 0
    for i in range(n - 1):
        for u, v, weight in edges:
            if costs[v] < costs[u] + weight:
                costs[v] = costs[u] + weight
    for u, v, weight in edges:
        if costs[v] < costs[u] + weight:
            return True
    return False

if __name__ == '__main__':
    for t in range(int(input())):
        n, m = (int(i) for i in input().split())
        edges = []
        for i in range(m):
            u, v, weight =  (int(j) for j in input().split())
            edges.append((u - 1, v - 1, weight))
        print('Yes' if bellman_ford(n, edges) else 'No')
