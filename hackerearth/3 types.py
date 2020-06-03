class DisjointSet:
    def __init__(self, n):
        self.parents = [i for i in range(n)]
    
    def union(self, x, y):
        root_x, root_y = self.find(x), self.find(y)
        if root_x != root_y:
            self.parents[root_x] = root_y

    def find(self, x):
        stack = []
        while x != self.parents[x]:
            stack.append(x)
            x = self.parents[x]
        while stack:
            self.parents[stack.pop()] = x
        return x

def kruskal(n, edges, gender_edges):
    dset = DisjointSet(n)
    cnt, gender_cnt = 0, 0
    for v, u in edges:
        if dset.find(v) != dset.find(u):
            dset.union(v, u)
            cnt += 1
    for v, u in gender_edges:
        if dset.find(v) != dset.find(u):
            dset.union(v, u)
            gender_cnt += 1
    return cnt, gender_cnt

if __name__ == '__main__':
    n, m = map(int, input().split())
    edges, man_edges, woman_edges = [], [], []
    for i in range(m):
        v, u, edge_type = (int(j) - 1 for j in input().split())
        if edge_type == 2:
            edges.append((v, u))
        elif edge_type == 1:
            man_edges.append((v, u))
        else:
            woman_edges.append((v, u))
    cnt, man_cnt = kruskal(n, edges, man_edges)
    cnt, woman_cnt = kruskal(n, edges, woman_edges)
    if cnt + man_cnt < n - 1 or cnt + woman_cnt < n - 1:
        print(-1)
    else:
        print(m - cnt - woman_cnt - man_cnt)
