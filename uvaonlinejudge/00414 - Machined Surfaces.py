def read_test_case():
    while True:
        n = int(input())
        if n == 0:
            break
        yield n, [input() for i in range(n)]

if __name__ == '__main__':
    for n, image in read_test_case():
        total, min_cnt = 0, 25
        for row in image:
            cnt = 0
            for ch in row:
                cnt += 1 if ch == ' ' else 0
            min_cnt = cnt if cnt < min_cnt else min_cnt
            total += cnt
        print(total - min_cnt * n)
