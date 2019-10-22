def inorder(pairs, i, n, value):
    if i < n:
        value = inorder(pairs, (i + 1) * 2 - 1, n, value)
        pairs[i][1] = value
        value = inorder(pairs, (i + 1) * 2 + 1 - 1, n, value + 1)
    return value

def min_swaps_to_sort(array, n):
    pairs.sort(key=lambda pair: pair[0])
    i, cnt = 0, 0
    while i < n:
        j = pairs[i][1]
        if i != j:
            pairs[i][1] = pairs[j][1]
            pairs[j][1] = j
            cnt += 1
        else:
            i += 1
    return cnt

if __name__ == '__main__':
    n = int(input())
    pairs = [[int(i), 0] for i in input().split()]
    inorder(pairs, 0, n, 0)
    print(min_swaps_to_sort(pairs, n))
