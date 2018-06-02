from operator import itemgetter

class DisjointSet:
    def __init__(self, n):
        self._parent = [i for i in range(n)]
        self._size = [1] * n

    def find(self, x):
        if self._parent[x] != x:
            self._parent[x] = self.find(self._parent[x])
        return self._parent[x]

    def union(self, x, y):
        root_x, root_y = self.find(x), self.find(y)
        if self._size[root_x] > self._size[root_y]:
            self._parent[root_y] = root_x
            self._size[root_x] += self._size[root_y]
        else:
            self._parent[root_x] = root_y
            self._size[root_y] += self._size[root_x]

def read_test_case():
    input()
    n, m = int(input()), int(input())
    dic, edge_list = {}, []
    idx = 0
    for i in range(m):
        source, target, w = input().split()
        for name in (source, target):
            if name not in dic:
                dic[name] = idx
                idx += 1
        edge_list.append((dic[source], dic[target], int(w)))
    return n, edge_list

def kruskal(n, edge_list):
    ds = DisjointSet(n)
    cost = 0
    for v, u, w in sorted(edge_list, key=itemgetter(2)):
        if ds.find(v) != ds.find(u):
            ds.union(v, u)
            cost += w
            if ds._size[ds.find(v)] == n:
                break
    return cost

if __name__ =='__main__':
    t = int(input())
    for i in range(t):
        if i > 0:
            print()
        print(kruskal(*read_test_case()))