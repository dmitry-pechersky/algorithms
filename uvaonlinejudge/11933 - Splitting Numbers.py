if __name__ == '__main__':
    while True:
        n = int(input())
        if n == 0:
            break
        a, b, order, pos = 0, 0, 0, 0
        while n >> pos > 0:
            if n & (1 << pos) > 0:
                a |= (order ^ 1) << pos
                b |= (order & 1) << pos
                order ^= 1
            pos += 1
        print(a, b)
