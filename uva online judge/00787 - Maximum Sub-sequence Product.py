import sys

def read_test_case():
    line = sys.stdin.readline().strip()
    if line == '':
        return None
    return [int(i) for i in line.split() if int(i) != -999999]

def max_subarray_product(array):
    n = len(array)
    max_product, min_value, max_value = array[0], array[0], array[0]
    for i in range(1, n):
        max_value, min_value = max(max_value * array[i], min_value * array[i], array[i]), min(max_value * array[i], min_value * array[i], array[i])
        if max_value > max_product:
            max_product = max_value
    return max_product

if __name__ == '__main__':
    for array in iter(read_test_case, None):
        print(max_subarray_product(array))
