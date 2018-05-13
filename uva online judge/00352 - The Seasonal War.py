import sys

def read_test_case():
    line = sys.stdin.readline().strip()
    if line == '':
        return None
    n = int(line)
    grid = []
    for i in range(n):
        grid.append([int(i) for i in sys.stdin.readline().strip()])
    return n, grid

def fill(grid, n, x, y):
    if x < 0 or x >= n or y < 0 or y >= n or grid[x][y] != 1:
        return
    grid[x][y] = 2
    for dx, dy in ((-1,0),(-1,1),(0,1),(1,1),(1,0),(1,-1),(0,-1),(-1,-1)):
        fill(grid, n, x + dx, y + dy)

if __name__ == '__main__':
    i = 1
    for n, grid in iter(read_test_case, None):
        eagles_cnt = 0
        for x in range(n):
            for y in range(n):
                if grid[x][y] == 1:                    
                    fill(grid, n, x, y)
                    eagles_cnt += 1
        print('Image number {} contains {} war eagles.'.format(i, eagles_cnt))
        i += 1
