import heapq

def read_test_case():
    input()
    n, e, t, m  = (int(input()) for i in range(4)) 
    adj_list = [[] for i in range(n + 1)]
    for i in range(m):
        a, b, weight = (int(i) for i in input().split())
        adj_list[b].append((a, weight))
    return n, e, t, adj_list

def dijkstra(n, start, max_cost, adj_list):
    costs, queue, cnt = [None] * (n + 1), [], 0
    costs[start] = 0
    heapq.heappush(queue, (0, start))
    while len(queue) > 0:
        cost, v = heapq.heappop(queue)
        if cost > max_cost:
            break
        if cost > costs[v]:
            continue
        cnt += 1
        for u, weight in adj_list[v]:
            if costs[u] is None or costs[u] > cost + weight:
                costs[u] = cost + weight
                heapq.heappush(queue, (cost + weight, u))

    return cnt

if __name__ == '__main__':
    for i in range(int(input())):
        if i > 0:
            print()
        print(dijkstra(*read_test_case()))