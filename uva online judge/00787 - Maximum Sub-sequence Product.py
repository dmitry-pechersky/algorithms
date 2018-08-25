import sys

def read_test_case():
    line = sys.stdin.readline().strip()
    if line == '':
        return None
    return [int(i) for i in line.split() if int(i) != -999999]

def max_subarray_product(array):
    n = len(array)
    min_array, max_array = [0] * n, [0] * n
    min_array[0], max_array[0] = array[0], array[0]
    for i in range(1, n):
        max_array[i] = max(max_array[i - 1] * array[i], min_array[i - 1] * array[i], array[i])
        min_array[i] = min(max_array[i - 1] * array[i], min_array[i - 1] * array[i], array[i])
    return max(max_array)

if __name__ == '__main__':
    for array in iter(read_test_case, None):
        print(max_subarray_product(array))
