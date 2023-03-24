def read_test_case():
    input()
    return input()

def kmp_prefix_function(s):
    n = len(s)
    pf = [0] * n
    j = 0
    for i in range(1, n):
        while j > 0 and s[i] != s[j]:
            j = pf[j - 1]
        if s[i] == s[j]:
            j += 1
        else:
            j = 0
        pf[i] = j
    return pf

if __name__ == '__main__':
    t = int(input())
    while(t > 0):
        s = read_test_case()
        n = len(s)
        p = n - kmp_prefix_function(s)[n - 1]
        print(p if p > 0 and n % p == 0 else n)
        if t > 1:
            print()
        t -= 1
        