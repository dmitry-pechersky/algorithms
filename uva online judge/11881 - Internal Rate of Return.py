def read_test_case():
    n = int(input())
    if n == 0:
        return None
    cf = [int(i) for i in input().split()]
    return  cf

def f(cf, x):  
    return cf[0] + sum(cf[i] / (1 + x) ** (i) for i in range(1, len(cf)))

def find_irr(cf, a, b):
    c = (b + a) / 2
    if b - a < 0.0001:
        return b
    if f(cf, c) < 0:
        return find_irr(cf, a, c)
    return find_irr(cf, c, b)

if __name__ == '__main__':
    for cf in iter(read_test_case, None):
        print("{:.2f}".format(find_irr(cf, -1, 1000000)))
