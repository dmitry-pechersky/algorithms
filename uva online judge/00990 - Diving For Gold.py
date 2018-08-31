import sys

def read_test_case():
    line = sys.stdin.readline()
    if line == '':
        return None
    t, w = (int(i) for i in line.split())
    n =  int(sys.stdin.readline())
    treasures = []
    for i in range(n):
        d, v = (int(i) for i in sys.stdin.readline().split())
        treasures.append((d, v, 3 * w * d))
    sys.stdin.readline()
    return t, w, n, treasures

def knapsack(t, w, n, treasures):
    cache = [[0] * (t + 1) for i in range(n)]
    d0, v0, t0 = treasures[0]
    for i in range(t0, t + 1):
        cache[0][i] = v0
    for i in range(1, n):
        di, vi, ti = treasures[i]
        for j in range(1, t + 1):
            if ti > j:
                cache[i][j] = cache[i - 1][j]
            else:
                cache[i][j] = max(cache[i - 1][j - ti] + vi, cache[i - 1][j])
    j = t
    selected_treasures = [False] * n
    for i in range(n - 1, 0, -1):
        di, vi, ti = treasures[i]
        if cache[i][j] != cache[i - 1][j]:
            j -= ti
            selected_treasures[i] = True
    if j > 0:
        selected_treasures[0] = True
    return cache[n - 1][t], selected_treasures

if __name__ == '__main__':
    for j, (t, w, n, treasures) in enumerate(iter(read_test_case, None)):
        gold_amount, selected_treasures = knapsack(t, w, n, treasures)
        if j > 0:
            print()
        print(gold_amount)
        print(sum(selected_treasures))
        for i in range(n):
            if selected_treasures[i]:
                di, vi, ti = treasures[i]
                print(di, vi)
