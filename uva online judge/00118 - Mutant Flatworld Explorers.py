import sys

directions = ['N', 'E', 'S', 'W']
dic = {'N': 0, 'E': 1, 'S': 2, 'W': 3}
dxy = [(0, 1), (1, 0), (0, -1), (-1, 0)]

def read_test_case():
    while True:
        line = sys.stdin.readline().split()
        if len(line) == 0:
            break
        x, y, direction = int(line[0]), int(line[1]), dic[line[2]]
        actions = sys.stdin.readline().strip()
        yield x, y, direction, actions

def walk(x, y, direction, actions, grid, X, Y):
    for action in actions:
        if action == 'L':
            direction = (direction - 1) % 4
        elif action == 'R':
            direction = (direction + 1) % 4
        else:
            dx, dy = dxy[direction]
            if x + dx > X or x + dx < 0 or y + dy > Y or y + dy < 0:
                if not grid[x][y]:
                    grid[x][y] = True
                    return x, y, direction, False
            else:
                x, y = x + dx, y + dy
    return x, y, direction, True

if __name__ == '__main__':
    X, Y = (int(i) for i in input().split())
    grid = [[False] * (Y + 1) for i in range(X + 1)]
    for x, y, direction, actions in read_test_case():
        x, y, direction, succeed = walk(x, y, direction, actions, grid, X, Y)
        if succeed:
            print(x, y, directions[direction])
        else:
            print(x, y, directions[direction], 'LOST')
