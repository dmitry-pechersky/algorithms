import sys

def read_test_case():
    line = sys.stdin.readline().strip()
    if len(line) == 0:
        return None
    return line

def kmp_prefix_function(string):
    n = len(string)
    prefix_func = [0] * n
    j = 0 
    for i in range(1, n):
        while j > 0 and string[i] != string[j]:
            j = prefix_func[j - 1]
        if string[i] == string[j]:
            j += 1
        else:
            j = 0
        prefix_func[i] = j
    return prefix_func

if __name__ == '__main__':
    for string in iter(read_test_case, None):
        n = len(string)
        prefix_func = kmp_prefix_function(''.join((string[::-1], '$', string)))
        print(string, string[:n - prefix_func[2 * n]][::-1], sep='')
