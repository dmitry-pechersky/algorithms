def read_test_case():
    n = int(input())
    matrix = [[None] * n for i in range(n)]
    i = 0
    while i < n * n:
        for value in input().split():
            matrix[i // n][i % n] = int(value)
            i += 1
    return n, matrix

def kadane(n, array):
    s, max_sum = array[0], array[0]
    for i in range(1, n):
        s = (s + array[i]) if 0 < s else  array[i]
        if s > max_sum:
            max_sum = s
    return max_sum

def maximum_sum_submatrix(n, matrix):
    max_sum = matrix[0][0]
    for left in range(n):
        array = [0] * n
        for right in range(left, n):
            for i in range(n):
                array[i] += matrix[i][right]
            max_sum = max(max_sum, kadane(n, array))
    return max_sum

if __name__ == '__main__':
    n, matrix = read_test_case()
    print(maximum_sum_submatrix(n, matrix))
