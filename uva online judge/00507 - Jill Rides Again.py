def read_test_case():
    s = int(input())
    roads = []
    for i in range(s - 1):
        roads.append(int(input()))
    return roads

def max_contiguous_subarray(array):
    n = len(array)
    for i in range(1, n):
        if array[i - 1] > 0:
            array[i] += array[i - 1]
    max_sum, max_i, max_j, i = array[0], 0, 0, 0
    for j in range(1, n):
        if array[j - 1] < 0:
            i = j
        if array[j] > max_sum or (array[j] == max_sum and j - i > max_j - max_i):
            max_sum, max_i, max_j = array[j], i, j
    return max_sum, max_i, max_j

if __name__ == '__main__':
    for i in range(int(input())):
        roads = read_test_case()
        if len(roads) == 0:
            print("Route {} has no nice parts".format(i + 1))
            continue
        max_sum, max_i, max_j = max_contiguous_subarray(roads)
        if max_sum > 0:
            print("The nicest part of route {} is between stops {} and {}".format(i + 1, max_i + 1, max_j + 2))
        else:
            print("Route {} has no nice parts".format(i + 1))
