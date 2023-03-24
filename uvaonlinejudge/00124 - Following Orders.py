import sys

def read_test_cases():
    while True:
        words = sorted(sys.stdin.readline().split())
        if len(words) == 0:
            break
        dic = {words[i]: i for i in range(len(words))}
        adj_list = [[] for i in range(len(words))]
        constraints = sys.stdin.readline().split()
        for i in range(0, len(constraints), 2):
            adj_list[dic[constraints[i + 1]]].append(dic[constraints[i]])
        yield adj_list, words

def topo_sorts(n, adj_list, visited, sort, func):
    if len(sort) == n:
        func(sort)
    else:
        for v in range(n):
            if not visited[v]:
                smallest = True
                for u in adj_list[v]:
                    if not visited[u]:
                        smallest = False
                        break
                if smallest:
                    visited[v] = True
                    sort.append(v)
                    topo_sorts(n, adj_list, visited, sort, func)
                    sort.pop()
                    visited[v] = False

if __name__ == '__main__':
    for i, (adj_list, words) in enumerate(read_test_cases()):
        n = len(words)
        if i > 0:
            print()
        topo_sorts(n , adj_list, [False] * n, [], lambda sort: print(*(words[v] for v in sort), sep=''))
