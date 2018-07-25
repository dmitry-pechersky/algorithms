import sys

def read_text(words, dic):
    line = sys.stdin.readline()
    if len(line) == 0:
        return None
    seq = []
    while line[0] != '#':
        for word in line.split():
            if word not in dic:
                dic[word] = len(words)
                words.append(word)
            seq.append(dic[word])
        line = sys.stdin.readline()
    return seq

def read_test_case():
    words, dic = [], {}
    seq1 = read_text(words, dic)
    if seq1 is None:
        return None
    seq2 = read_text(words, dic)
    return words, seq1, seq2

def lcs(seq1, seq2):
    n1, n2 = len(seq1), len(seq2)
    alignment = [[0] * (n2 + 1) for i in range(n1 + 1)]
    for i in range(1, n1 + 1):
        for j in range(1, n2 + 1):
            if seq1[i - 1] == seq2[j - 1]:
                alignment[i][j] = max(alignment[i - 1][j - 1] + 1, alignment[i][j - 1], alignment[i - 1][j])
            else:
                alignment[i][j] = max(alignment[i][j - 1], alignment[i - 1][j])
    subseq = []
    i, j = n1, n2
    while i > 0 and j > 0:        
        if alignment[i][j] == alignment[i - 1][j - 1] + 1 and seq1[i - 1] == seq2[j - 1]:
            i, j = i - 1, j - 1
            subseq.append(seq1[i])
        elif alignment[i][j] == alignment[i - 1][j]:
            i = i - 1
        else:
            j = j - 1
    return subseq

if __name__ == '__main__':
    for words, seq1, seq2 in iter(read_test_case, None):
        print(*(words[i] for i in  reversed(lcs(seq1, seq2))))