from sys import stdin 

def dfs(x, y, grid):
    n, m = len(grid), len(grid[0])
    stack = [(x, y)]
    cnt = 1
    grid[x][y] = 1
    while stack:
        x, y = stack.pop()
        for x1, y1, in ((x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)):
            if 0 <= x1 < n and 0 <= y1 < m and grid[x1][y1] == 0:
                stack.append((x1, y1))
                grid[x1][y1] = 1
                cnt += 1    
    return cnt

if __name__ == '__main__':
    t, _  = int(stdin.readline()), stdin.readline()
    for i in range(t):
        x, y = map(int, stdin.readline().split())
        grid = []
        while True:
            line = stdin.readline().strip()
            if len(line) == 0:
                break
            grid.append(list(map(int, line)))
        if i > 0:
            print()
        print(dfs(x - 1, y - 1, grid))
