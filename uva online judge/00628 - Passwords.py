import sys

def read_test_case():
    line = sys.stdin.readline().strip()
    if line == '':
        return None
    words = []
    n = int(line)
    for i in range(n):
        words.append(sys.stdin.readline().strip())
    rules = []
    n = int(sys.stdin.readline().strip())
    for i in range(n):
        rules.append(sys.stdin.readline().strip())
    return words, rules

def print_passwods_rec(rule, word, password, i):
    n = len(rule)
    if i < n:
        if rule[i] == '0':
            for j in range(10):
                password[i] = j
                print_passwods_rec(rule, word, password, i + 1)
        else:
            password[i] = word
            print_passwods_rec(rule, word, password, i + 1)
    else:
        print(*password, sep='')

if __name__ == '__main__':
    for words, rules in iter(read_test_case, None):
        print('--')
        for rule in rules:
            password = [None] * len(rule)
            for word in words:
                print_passwods_rec(rule, word, password, 0)
