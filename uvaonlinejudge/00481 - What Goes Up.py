import sys

def smallest_larger(array, dp, value):
    a, b = 0, len(dp) - 1
    while a != b:
        c = int((a + b) / 2)
        if array[dp[c]] >= value:
            b = c
        else:
            a = c + 1
    return b

def longest_increasing_subsequence(array):
    dp, parent = [0], [None] * len(array)
    for i in range(1, len(array)):
        if array[i] > array[dp[-1]]:
            parent[i] = dp[-1]
            dp.append(i)
        elif array[i] < array[dp[-1]]:
            j = smallest_larger(array, dp, array[i])
            if array[dp[j]] != array[i]:
                dp[j] = i
                if j > 0:
                    parent[i] = dp[j - 1]
    for i in range(len(dp) - 2, -1, -1):
        dp[i] = parent[dp[i + 1]]
    return len(dp), dp

if __name__ == '__main__':
    array = [int(line) for line in sys.stdin.readlines()]
    length, dp = longest_increasing_subsequence(array)
    print('{}\n-'.format(length))
    print(*(array[i] for i in dp), sep='\n')
