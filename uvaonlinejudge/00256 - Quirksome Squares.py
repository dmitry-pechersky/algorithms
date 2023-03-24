import sys

def quirksome_squares(n):
    i, i2, ten_power_half_n = 0, 0, 10 ** (n / 2)
    nums = []
    while i2 < 10 ** n:
        if i2 % ten_power_half_n + i2 // ten_power_half_n == i:
            nums.append(i2)
        i += 1
        i2 = i * i
    return nums

if __name__ == '__main__':
    max_n = 8
    cache = [None] * (max_n // 2)
    for n in (int(i) for i in sys.stdin.readlines()):
        if cache[n // 2 - 1] is None:
            cache[n // 2 - 1] = quirksome_squares(n)
        for i in cache[n // 2 - 1]:
            print('{0:0{width}}'.format(i, width=n))
