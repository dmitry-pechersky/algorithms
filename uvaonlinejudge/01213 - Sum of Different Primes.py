def prime_numbers(max_value):
    array = [True] * (max_value + 1)
    for i in range(2, max_value):
        if array[i]:
            for j in range(2 * i, max_value + 1, i):
                array[j] = False
    return [i for i in range(2, len(array)) if array[i]]

def knapsack(n, k, numbers):
    dp = [[0] * (k + 1) for i in range(n + 1)]
    for number in numbers:
        for i in range(n, 1, -1):
            if number == i:
                dp[i][1] = 1
            elif number < i:
                for j in range(2, k + 1):
                    dp[i][j] += dp[i - number][j - 1]
    return dp

if __name__ == '__main__':
    max_n, max_k = 1120, 14
    dp = knapsack(max_n, max_k, prime_numbers(max_n))
    while True:
        n, k = (int(i) for i in input().split())
        if n == 0 and k == 0:
            break
        print(dp[n][k])
        
    