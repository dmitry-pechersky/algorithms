def longest_polindrom(string):
    n = len(string)
    if n == 0:
        return 0
    length = [[1] * n for i in range(n)]
    max_length = 1
    for i in range(n - 1):
        if string[i] == string[i + 1]:
            length[i][i + 1] = 2
            max_length = 2
    for l in range(3, n + 1):
        for i in range(n - l + 1):
            if string[i] == string[i + l - 1]:
                length[i][i + l - 1] = length[i + 1][i + l - 2] + 2 
            else:
                length[i][i + l - 1] = max(length[i + 1][i + l - 1], length[i][i + l - 2])
            max_length = max(max_length, length[i][i + l - 1])
    return max_length
        
if __name__ == '__main__':
    for i in range(int(input())):
        string = input().strip()
        print(longest_polindrom(string))
