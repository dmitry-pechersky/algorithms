import heapq

def argus(queries, k):
    queue, ids = [], []
    for id, period in queries:
        heapq.heappush(queue, (period, id, period))
    for i in range(k):
        time, id, period = heapq.heappop(queue) 
        heapq.heappush(queue, (time + period, id, period))
        ids.append(id)
    return ids

if __name__ == '__main__':
    queries = []
    while True:
        s = input().strip()
        if s == '#':
            break
        _, id, period = s.split()
        queries.append((int(id), int(period)))
    k = int(input())
    for i in argus(queries, k):
        print(i)