if __name__ == '__main__':
    n, m = (int(i) for i in input().split())
    array = [0] * n
    for i in range(m):
        a, b, k = (int(i) for i in input().split())
        array[a - 1] += k
        if b < n:
            array[b] -= k
    max_value, value = 0, 0
    for i in range(n):
        value += array[i]
        if value > max_value:
            max_value = value
    print(max_value)
