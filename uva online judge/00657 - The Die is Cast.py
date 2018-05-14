import sys

dxy = ((1, 0), (0, 1), (-1, 0), (0, -1))

def read_test_case():
    w, h = [int(i) for i in input().split()]
    if w == 0:
        return None
    grid = []
    for i in range(h):
        grid.append(list(input().strip()))
    return grid, h, w

def fill_dice(grid, h, w, x, y):
    if x < 0 or x >= h or y < 0 or y >= w or grid[x][y] == '.':
        return 0
    numbers_cnt = 0
    if grid[x][y] == 'X':
        fill_number(grid, h, w, x, y)
        numbers_cnt = 1
    grid[x][y] = '.'
    for dx, dy in dxy:
        numbers_cnt += fill_dice(grid, h, w, x + dx, y + dy)
    return numbers_cnt

def fill_number(grid, h, w, x, y):
    if x < 0 or x >= h or y < 0 or y >= w or grid[x][y] != 'X':
        return 
    grid[x][y] = '*'
    for dx, dy in dxy:
        fill_number(grid, h, w, x + dx, y + dy)


if __name__ == '__main__':
    sys.setrecursionlimit(10000)
    case_i = 1
    for grid, h, w in iter(read_test_case, None):
        numbers = []
        for x in range(h):
            for y in range(w):
                if grid[x][y] != '.':
                    num = fill_dice(grid, h, w, x, y)
                    if num > 0:
                        numbers.append(num)
        print('Throw', case_i)
        print(*sorted(numbers))
        print()
        case_i += 1

        