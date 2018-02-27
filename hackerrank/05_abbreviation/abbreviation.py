def abbreviation(str1, str2):
    n1, n2 = len(str1), len(str2)
    alignment = [[None] * (n2 + 1) for i in range(n1 + 1)]
    alignment[0][0] = 0
    for i in range(1, n1 + 1):
        if str1[i - 1].isupper():
            break
        alignment[i][0] = 0
    for j in range(1, n2 + 1):
        for i in range(1, n1 + 1):
            if alignment[i - 1][j - 1] != None and (str1[i - 1] == str2[j - 1] or str1[i - 1].upper()== str2[j - 1]):
                alignment[i][j] = alignment[i - 1][j - 1] + 1
            if alignment[i - 1][j] != None and  str1[i - 1].islower():
                if  alignment[i][j] == None or alignment[i][j] < alignment[i - 1][j]:
                    alignment[i][j] = alignment[i - 1][j]
    return alignment[n1][n2] != None

t = int(input())
for i in range(t):
    str1 = input().strip()
    str2 = input().strip()
    print('YES' if abbreviation(str1, str2) else 'NO')