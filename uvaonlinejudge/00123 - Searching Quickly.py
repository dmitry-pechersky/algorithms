import sys

def read_test_case():
    ignore, titles = set(), []
    while True:
        line = sys.stdin.readline().strip()
        if line == '::':
            break
        ignore.add(line)
    while True:
        line = sys.stdin.readline().strip().lower().split()
        if len(line) == 0:
            break
        titles.append(line)
    return ignore, titles

if __name__ == '__main__':
    ignore, titles = read_test_case()
    kwic = []
    for title in titles:
        for index, word in enumerate(title):
            if word not in ignore:
                kwic.append((word, index, title))
    kwic = sorted(kwic, key=lambda value: value[0])
    for keyword, index, title in kwic:
        print(*(word.upper() if index == i else word for i, word in enumerate(title)))
