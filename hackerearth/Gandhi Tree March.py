def parse(tree, pos, column, c, array):
    if tree[pos] == '.':
        pos += 1
    else:
        if column == c:
            array.append(tree[pos])
        pos = parse(tree, pos + 2, column - 1, c, array)
        pos = parse(tree, pos, column + 1, c, array)
        pos += 1
    return pos

if __name__ == '__main__':
    for i in range(int(input())):
        c, tree = input().split()
        array = []
        parse(tree, 0, 0, int(c), array)
        if len(array) > 0:
            print(*sorted(array), sep='')
        else:
            print('Common Gandhijee!')
