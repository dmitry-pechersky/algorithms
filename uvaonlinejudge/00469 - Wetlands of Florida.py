import sys

def flood_fill(grid, n, m, i, j, ref_i, ref_j):
    if i < 0 or i >= n or j < 0 or j >= m or grid[i][j] != 'W':
        return 0
    grid[i][j] = (ref_i, ref_j)
    cnt = 1
    for di, dj in ((-1,0),(-1,1),(0,1),(1,1),(1,0),(1,-1),(0,-1),(-1,-1)):
        cnt += flood_fill(grid, n, m, i + di, j + dj, ref_i, ref_j)
    return cnt

def read_test_case():
    grid, queries = [], []
    while True:
        line = sys.stdin.readline().strip()
        if line =='':
            break
        if line[0] in ('L','W'):
            grid.append(list(line))
        else:
            queries.append([int(i) - 1 for i in line.split()])
    return grid, len(grid), len(grid[0]), queries

if __name__ == '__main__':
    sys.setrecursionlimit(100000)
    t = int(sys.stdin.readline())
    sys.stdin.readline()
    for k in range(t, 0, -1):
        grid, n, m, queries = read_test_case()
        sizes_cache = [[0] * m for i in range(n)]
        for i, j in queries:
            if grid[i][j] == 'L':
                print(0)
            elif grid[i][j] == 'W':
                sizes_cache[i][j] = flood_fill(grid, n, m, i, j, i, j)
                print(sizes_cache[i][j])
            else:
                ref_i, ref_j = grid[i][j]
                print(sizes_cache[ref_i][ref_j])
        if k > 1:
            print()

                
