def read_test_case():
    while True:
        n = int(input())
        if n == 0:
            break
        stones = [[int(j) for j in input().split()] for i in range(n)]
        input()
        yield n, stones

class DisjointSet:
    def __init__(self, n):
        self.parent = [i for i in range(n)]
    
    def union(self, x, y):
        root_x, root_y = self.find(x), self.find(y)
        if root_x != root_y:
            self.parent[root_x] = root_y
    
    def find(self, x):
        if self.parent[self.parent[x]] != self.parent[x]:
            self.parent[x] = self.find(self.parent[x])
        return self.parent[x]

def kruskal(edges, n, start, end):
    disjoint_set = DisjointSet(n)
    for weight, v, u in edges:
        disjoint_set.union(v, u)
        if disjoint_set.find(start) == disjoint_set.find(end):
            return weight

if __name__ == '__main__':
    for t, (n, stones) in enumerate(read_test_case(), 1):
        edges = [((stones[i][0] - stones[j][0]) ** 2 + (stones[i][1] - stones[j][1]) ** 2, i , j) for i in range(n - 1) for j in range(i + 1, n)]
        edges.sort()
        print('Scenario #{}'.format(t))
        print('Frog Distance = {:.3f}'.format(kruskal(edges, n, 0, 1) ** 0.5), end='\n\n')
