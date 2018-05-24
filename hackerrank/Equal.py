def calc_cache(n=1002):
    cache = [None] * n
    cache[0] = 0
    for i in range(n):
        for op in (1, 2, 5):
            if i + op < n and (cache[i + op] == None or cache[i + op] > cache[i] + 1):
                cache[i + op] = cache[i] + 1
    return cache

def calc_operations_(chocs, cache, dif):
    res = 0
    for num in chocs:
        res += cache[num - dif]
    return res

def calc_operations(chocs, cache):
    min_chocs = 1000
    for num in chocs:
        min_chocs = min_chocs if num > min_chocs else num
    return min(calc_operations_(chocs, cache, min_chocs), calc_operations_(chocs, cache, min_chocs - 1), calc_operations_(chocs, cache, min_chocs - 2))

cache = calc_cache()
t = int(input())
for i in range(t):
    n = int(input())
    chocs = [int(i) for i in input().split()]
    print(calc_operations(chocs, cache))