from operator import itemgetter
import sys

class DisjointSet():
    def __init__(self, n):
        self._parents = [i for i in range(n)]
        self._sizes = [1] * n

    def find(self, x):
        if self._parents[x] != x:
            self._parents[x] = self.find(self._parents[x])
        return self._parents[x]

    def union(self, x, y):
        root_x, root_y = self.find(x), self.find(y)
        if root_x != root_y:
            if self._sizes[root_x] >  self._sizes[root_y]:
                self._parents[root_y] = root_x
                self._sizes[root_x] += self._sizes[root_y]
            else:
                self._parents[root_x] = root_y
                self._sizes[root_y] += self._sizes[root_x]                

def read_test_case():
    line = sys.stdin.readline().strip()
    if line == '':
        return None
    cost, edge_list, n = 0, [], int(line)
    for i in range(n - 1):
        v, u, w = (int(j) for j in sys.stdin.readline().split())
        cost += w
    for i in range(int(sys.stdin.readline())):
        edge_list.append([int(j) for j in sys.stdin.readline().split()])
    for i in range(int(sys.stdin.readline())):
        edge_list.append([int(j) for j in sys.stdin.readline().split()])
    sys.stdin.readline()
    return n, cost, edge_list
    
def kruskal(n, edge_list):
    ds = DisjointSet(n + 1)
    cost = 0
    for v, u, w in sorted(edge_list, key=itemgetter(2)):
        if ds.find(v) != ds.find(u):
            ds.union(v, u)
            cost += w
            if ds._sizes[ds.find(v)] == n:
                break
    return cost

if __name__ == '__main__':
    for i, (n, cost, edge_list) in enumerate(iter(read_test_case, None)):
        if i > 0:
            print()
        print(cost, kruskal(n, edge_list), sep='\n')
        