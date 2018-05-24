def lcs(seq1, seq2):
    n1, n2 = len(seq1), len(seq2)
    alignment = [[0] * (n2 + 1) for i in range(n1 + 1)]
    for i in range(1, n1 + 1):
        for j in range(1, n2 + 1):
            match = alignment[i - 1][j - 1] + 1
            del1 = alignment[i - 1][j]
            del2 = alignment[i][j - 1]
            if seq1[i - 1] == seq2[j - 1]:
                alignment[i][j] = max(match, del1, del2)
            else:
                alignment[i][j] = max(del1, del2)
    i, j = n1, n2
    marks = [0] * n1
    while True:
        if alignment[i][j] == alignment[i - 1][j]:
            i -= 1
        elif alignment[i][j] == alignment[i][j - 1]:
            j -= 1
        else:
            marks[i - 1] = 1
            i, j = i - 1, j - 1
        if i == 0 or j == 0:
            break
    return  [seq1[i] for i in range(n1) if marks[i] == 1]

n1, n2 = [int(i) for i in input().split()]
seq1 = [int(i) for i in input().split()]
seq2 = [int(i) for i in input().split()]
print(*lcs(seq1, seq2), sep=' ')
