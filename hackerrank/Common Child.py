def lcs(str1, str2):
    n1, n2 = len(str1), len(str2)
    prev, cur = [0] * (n2 + 1), [0] * (n2 + 1)
    for i in range(1, n1 + 1):
        for j in range(1, n2 + 1):
            cur[j] = cur[j - 1]
            if cur[j] < prev[j]:
                cur[j] = prev[j]
            if str1[i - 1] == str2[j - 1] and cur[j] < prev[j - 1] + 1:
                cur[j] = prev[j - 1] + 1
        prev, cur = cur, prev
    return prev[n2]

if __name__ == '__main__':
    str1, str2 = input(), input()
    print(lcs(str1, str2))
