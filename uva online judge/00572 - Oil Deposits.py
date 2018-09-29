def read_test_cases():
    while True:
        m, n = (int(i) for i in input().split())
        if m == 0:
            break
        grid = [list(input().strip()) for i in range(m)]
        yield m, n, grid

def dfs(m, n, grid, x, y):
    stack, grid[x][y] = [(x, y)], '*'
    while len(stack) > 0:
        x, y = stack.pop()
        for next_x, next_y in ((x + dx, y + dy) for dx in (-1, 0, 1) for dy in (-1, 0, 1)):
            if next_x >= 0 and next_x < m and next_y >= 0 and next_y < n and grid[next_x][next_y] == '@':
                grid[next_x][next_y] = '*'
                stack.append((next_x, next_y))
    
def connected_components(m, n, grid):
    cnt = 0
    for x in range(m):
        for y in range(n):
            if grid[x][y] == '@':
                dfs(m, n, grid, x, y)
                cnt += 1
    return cnt

if __name__ == '__main__':
    for m, n, grid in read_test_cases():
        print(connected_components(m, n, grid))
