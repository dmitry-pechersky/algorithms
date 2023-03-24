import sys

def read_test_case():
    while True:
        line = sys.stdin.readline().strip()
        if len(line) == 0:
            break
        yield line

if __name__ == '__main__':
    n = int(input())
    sys.stdin.readline()
    for case in range(n):
        fragments, cnts, min_len, max_len = [[] for i in range(256)], {}, 256, 0
        for fragment in read_test_case():
            fragments[len(fragment)].append(fragment)
            if len(fragment) > max_len:
                max_len = len(fragment)
            if len(fragment) < min_len:
                min_len = len(fragment)
        length = min_len + max_len                
        for i in range(min_len, (length + 1) // 2):
            for j in fragments[i]:
                for k in fragments[length - i]:
                    cnts[j + k] = cnts.get(j + k, 0) + 1
                    cnts[k + j] = cnts.get(k + j, 0) + 1
        if length % 2 == 0:
            middle_fragments = fragments[length // 2]
            for j in range(len(middle_fragments)):
                for k in range(j + 1, len(middle_fragments)):
                    cnts[middle_fragments[j] + middle_fragments[k]] = cnts.get(middle_fragments[j] + middle_fragments[k], 0) + 1
                    cnts[middle_fragments[k] + middle_fragments[j]] = cnts.get(middle_fragments[k] + middle_fragments[j], 0) + 1
        max_value = None
        for key in cnts:
            if max_value is None or cnts[key] > cnts[max_value]:
                max_value = key
        if case > 0:
            print()
        print(max_value)
