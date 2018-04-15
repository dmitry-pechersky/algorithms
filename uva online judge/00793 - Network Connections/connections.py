class DisjointSet:
    def __init__(self, n):
        self._parents = [i for i in range(n)]
        self._ranks = [0] * n
        self.n = n

    def find(self, x):
        if self._parents[x] != x:
            self._parents[x] = self.find(self._parents[x])
        return self._parents[x]
    
    def union(self, x, y):
        x_root = self.find(x)
        y_root = self.find(y)
        if x_root == y_root:
            return
        if self._ranks[x_root] > self._ranks[y_root]:
            self._parents[y_root] = x_root
        else:
            self._parents[x_root] = y_root
            if self._ranks[x_root] == self._ranks[y_root]:
                self._ranks[y_root] += 1    

if __name__ == '__main__':
    t = int(input())
    input()
    for i in range(t):
        n = int(input())
        djset = DisjointSet(n)
        successful_cnt, unsuccessful_cnt = 0, 0 
        while True:
            s = input().strip()
            if s == '':
                break
            op, source, dest, = s.split()
            if op == 'c':
                djset.union(int(source) - 1, int(dest) - 1)
            else:
                if djset.find(int(source) - 1) == djset.find(int(dest) - 1):
                    successful_cnt += 1
                else:
                    unsuccessful_cnt += 1
        print(successful_cnt, ',', unsuccessful_cnt, sep='')


