import sys

def read_test_case():
    str1 = sys.stdin.readline()
    if len(str1) == 0:
        return None
    str1 = str1.strip()
    str2 = sys.stdin.readline().strip()
    return str1, str2

def edit_distance(str1, str2):
    n1, n2 = len(str1) + 1, len(str2) + 1
    alignment = [[0] * n2 for i in range(n1)]
    for i in range(1, n1):
        alignment[i][0] = i
    for i in range(1, n2):
        alignment[0][i] = i
    for i in range(1, n1):
        for j in range(1, n2):
            delete = alignment[i-1][j] + 1
            insert = alignment[i][j-1] + 1
            match = alignment[i-1][j-1]
            mismatch = alignment[i-1][j-1] + 1
            alignment[i][j] = min(delete, insert, match if str1[i-1] == str2[j-1] else mismatch)
    commands = []
    i , j = n1 - 1, n2 - 1
    while i > 0 or j > 0:
        if j > 0 and alignment[i][j] == alignment[i][j-1] + 1:
            commands.append(('Insert', i, str2[j-1]))
            j = j - 1
        elif i > 0 and alignment[i][j] == alignment[i-1][j] + 1:
            commands.append(('Delete', i - 1, None))
            i = i - 1
        elif i > 0 and j > 0 and alignment[i][j] == alignment[i-1][j-1] and str1[i-1] == str2[j-1]:
            i, j = i - 1, j - 1
        elif i > 0 and j > 0 and alignment[i][j] == alignment[i-1][j-1] + 1:
            commands.append(('Replace', i - 1, str2[j-1]))
            i, j = i - 1, j - 1
    return alignment[n1 - 1][n2 - 1], commands

if __name__ == '__main__':
    for i, (str1, str2) in enumerate(iter(read_test_case, None)):
        dist, commands = edit_distance(str1, str2)
        if i > 0:
            print()
        print(dist)
        bias = 0
        for j in range(len(commands)):
            command, pos, ch = commands[len(commands) - j - 1]
            print(j + 1, command, pos + 1 + bias, end='')
            if ch is not None:
                print(',{}'.format(ch))
            else:
                print()
            if command == 'Insert':
                bias += 1
            elif command == 'Delete':
                bias -= 1            

