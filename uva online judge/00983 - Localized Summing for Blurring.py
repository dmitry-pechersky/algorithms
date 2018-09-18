import sys

def read_test_case(matrix):
    while True:
        line = sys.stdin.readline().strip()
        if line == '':
            break
        n, m = (int(i) for i in line.split()) 
        for i in range(n):
            for j in range(n):
                matrix[i][j] = int(sys.stdin.readline())
                if i > 0:
                    matrix[i][j] += matrix[i - 1][j]
                if j > 0:
                    matrix[i][j] += matrix[i][j - 1]
                if i > 0 and j > 0:
                    matrix[i][j] -= matrix[i - 1][j - 1]
        sys.stdin.readline()
        yield n, m

if __name__ == '__main__':
    matrix = [[0] * 1000 for i in range(1000)]
    for i, (n, m) in enumerate(read_test_case(matrix)):
        total_sum = 0
        if i > 0:
            print()
        for i in range(n - m + 1):
            for j in range(n - m + 1):
                value = matrix[i + m - 1][j + m - 1]
                if i > 0:
                    value -= matrix[i - 1][j + m - 1]
                if j > 0:
                    value -= matrix[i + m - 1][j - 1]
                if i > 0 and j > 0:
                    value += matrix[i - 1][j - 1]
                total_sum += value
                print(value)
        print(total_sum)
