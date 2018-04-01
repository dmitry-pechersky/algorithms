class DisjointSet:
    def __init__(self, n):
        self.n = n
        self._parents = [i for i in range(n)]
        self._sizes = [1] * n
        self._ranks = [0] * n

    def find(self, x):
        if self._parents[x] != x:
            self._parents[x] = self.find(self._parents[x])
        return self._parents[x]

    def union(self, x, y):
        root_x , root_y = self.find(x), self.find(y)
        if root_x == root_y:
            return
        if self._ranks[root_x] > self._ranks[root_y]:
            self._parents[root_y] = root_x
            self._sizes[root_x] += self._sizes[root_y] 
        else:
            self._parents[root_x] = root_y
            self._sizes[root_y] += self._sizes[root_x] 
            if self._ranks[root_x] == self._ranks[root_y]:
                self._ranks[root_y] += 1

if __name__ == '__main__':
    while True:
        n, m = [int(i) for i in input().split()]
        if n == 0 and m == 0:
            break
        djset = DisjointSet(n)
        for j in range(m):
            group = [int(i) for i in input().split()[1:]]
            for student in group[1:]:
                djset.union(group[0], student)
        print(djset._sizes[djset.find(0)])



