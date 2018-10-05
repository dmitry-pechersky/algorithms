def cover(intervals):
    current_row, cnt = 0, 0
    for row, a, b in sorted(intervals):
        if current_row != row:
            current_b, current_row = 0, row
        if a <= current_b:
            if b > current_b:
                cnt += b - current_b
                current_b = b
        else:
            current_b = b
            cnt += b - a + 1
    return cnt

if __name__ == '__main__':
    n, m, k = (int(i) for i in input().split())
    intervals = [[int(i) for i in input().split()] for i in range(k)]
    print(n * m - cover(intervals))
