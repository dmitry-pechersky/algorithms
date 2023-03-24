from math import inf

def read_test_cases():
    while True:
        cuts = [0]
        l = int(input())
        if l == 0:
            break
        input()
        cuts.extend(int(i) for i in input().split())
        cuts.append(l)
        yield cuts

def cutting_sticks_dp(cuts, dp):
    n = len(cuts)
    for i in range(0, n - 1):
        dp[i][i + 1] = 0
    for l in range(2, n):
        for i in range(0, n - l):
            min_value = inf
            for k in range(i + 1, i + l):
                value = dp[i][k] + dp[k][i + l]
                if value < min_value:
                    min_value = value
            dp[i][i + l] = min_value + cuts[i + l] - cuts[i]
    return dp[0][n - 1]

if __name__ == '__main__':
    max_size = 51
    dp = [[None] * max_size for i in range(max_size)]
    for cuts in read_test_cases():
        print('The minimum cutting is {}.'.format(cutting_sticks_dp(cuts, dp)))
