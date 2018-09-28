def smallest_greater_dc(array, a, b, value):
    while a != b:
        c = (a + b) // 2
        if array[c] < value:
            a = c + 1
        else:
            b = c
    return b

def longest_increasing_subsequence(sequence):
    dp = [sequence[0]]
    for i in range(1, len(sequence)):
        if sequence[i] > dp[-1]:
            dp.append(sequence[i])
        elif sequence[i] < dp[-1]:
            dp[smallest_greater_dc(dp, 0, len(dp) - 1, sequence[i])] = sequence[i]
    return len(dp)

if __name__ == '__main__':
    sequence = [int(input()) for i in range(int(input()))]
    print(longest_increasing_subsequence(sequence))
