from collections import deque

def read_pairs():
    while True:
        line = [int(i) for i in input().split()]
        for a, b in ((line[i], line[i+1]) for i in range(0,len(line),2)):
            if a == 0 and b == 0:
                return
            yield a,b

def read_test_case():
    m = int(input())
    if m == 0:
        return None
    dic, words, queries = {}, [], []
    adj_list = []
    for a, b in read_pairs():
        if m > 0:
            for i in (a, b):
                if i not in dic:
                    words.append(i)
                    dic[i] = len(words) - 1
                    adj_list.append([])
            adj_list[dic[a]].append(dic[b])
            adj_list[dic[b]].append(dic[a])
            m -= 1
        else:
            if a not in dic:
                words.append(a)
                dic[a] = len(words) - 1
            queries.append((dic[a], b))
    input()
    return len(words), adj_list, queries, words

def bfs(n, adj_list, v, d):
    dist, queue = [None] * n, deque([v])
    dist[v], cnt = 0, 1
    while(len(queue) > 0):
        v = queue.popleft()
        if dist[v] >= d:
            break
        for u in adj_list[v]:
            if dist[u] is None:
                dist[u] = dist[v] + 1
                queue.append(u)
                cnt += 1
    return cnt
    
if __name__ == '__main__':
    i = 1
    for n, adj_list, queries, words in iter(read_test_case, None):
        for v, d in queries:
            cnt = bfs(n, adj_list, v, d)
            print("Case {}: {} nodes not reachable from node {} with TTL = {}.".format(i, n - cnt, words[v], d))
            i += 1

