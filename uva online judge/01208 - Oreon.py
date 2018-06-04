from operator import itemgetter

class DisjoitSet:
    def __init__(self, n):
        self._parent = [i for i in range(n)]
        self._size = [1] * n

    def find(self, x):
        if self._parent[x] != x:
            self._parent[x] = self.find(self._parent[x])
        return self._parent[x]

    def union(self, x, y):
        root_x = self.find(x)
        root_y = self.find(y)
        if self._size[root_x] > self._size[root_y]:
            self._parent[root_y] = root_x
            self._size[root_x] += self._size[root_y]
        else:
            self._parent[root_x] = root_y
            self._size[root_y] += self._size[root_x]

def read_test_case():
    n = int(input())
    edge_list = []
    for i in range(n):
        weights = [int(w) for w in input().split(sep=', ')]
        for j in range(i + 1, n):
            if weights[j] > 0:
                edge_list.append((i, j, weights[j]))
    return n, edge_list

def kruskal(n, edge_list):
    ds = DisjoitSet(n)
    mst_edge_list = []
    for v, u, w in sorted(edge_list, key=itemgetter(2)):
        if ds.find(v) != ds.find(u):
            ds.union(v, u)
            mst_edge_list.append((v, u, w))
            if len(mst_edge_list) == n - 1:
                break
    return mst_edge_list

if __name__ == '__main__':
    for i in range(int(input())):
        n, edge_list = read_test_case()
        print('Case {}:'.format(i + 1))
        for v, u, w in kruskal(n, edge_list):
            print('{}-{} {}'.format(chr(65 + v), chr(65 + u), w))