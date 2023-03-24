import sys

def read_numbers():
    for line in sys.stdin:
        for value in line.split():
            yield int(value)

def read_test_cases():
    iterator = read_numbers()
    while True:
        n = next(iterator, 0)
        if n == 0:
            break
        yield [next(iterator, 0) for i in range(n)]

def max_subarray_sum(array):
    current_sum = array[0] if array[0] > 0 else 0
    max_sum = current_sum
    for i in range(1, len(array)):
        current_sum = current_sum + array[i] if current_sum + array[i] > 0 else 0
        if max_sum < current_sum:
            max_sum = current_sum
    return max_sum

if __name__ == '__main__':
    for nums in read_test_cases():
        max_sum = max_subarray_sum(nums)
        if max_sum > 0:
            print('The maximum winning streak is {}.'.format(max_sum))
        else:
            print("Losing streak.")
