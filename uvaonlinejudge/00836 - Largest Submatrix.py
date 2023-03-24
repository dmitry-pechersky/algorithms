def read_test_case():
    input()
    matrix = [input().strip()]
    for i in range(len(matrix[0]) - 1):
        matrix.append(input().strip())
    return matrix

def largest_subarray(n, value, array):
    size, max_size = 0, 0
    for i in range(n):
        if array[i] == value:
            size += 1
            if size > max_size:
                max_size = size
        else:
            size = 0
    return max_size

def largest_submatrix(matrix):
    n = len(matrix)
    max_size = 0
    for left in range(n):
        array = [0] * n
        for right in range(left, n):
            for i in range(n):
                if matrix[right][i] == '1':
                    array[i] += 1
            max_size = max(max_size, largest_subarray(n, right - left + 1, array) * (right - left + 1))
    return max_size
            
if __name__ == '__main__':
    for i in range(int(input())):
        if i > 0:
            print()
        print(largest_submatrix(read_test_case()))
