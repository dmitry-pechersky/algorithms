def insert(node, string):
    i, cnt, n = 0, 0, len(string)
    for char in string:
        j = ord(char) - 97
        if node[j] is None:
            node[j] = [None] * 26
            node = node[j]
            cnt += 1
        else:
            node = node[j]
    return cnt

if __name__ == '__main__':
    cnt, node = 1, [None] * 26
    for i in range(int(input())):
        cnt += insert(node, input())
    print(cnt)
