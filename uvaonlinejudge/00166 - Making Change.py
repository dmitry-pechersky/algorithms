from math import inf

def read_test_case():
    line = input().split()
    coins = [int(i) for i in line[0: -1]]
    if sum(coins) == 0:
        return None
    return coins, int(float(line[-1]) * 20)

def coin_change_greedy(denomination, transaction):
    cnt = 0
    for d in reversed(denomination):
        cnt += transaction // d
        transaction = transaction % d
    return cnt

def coin_change_dp(coins, denomination, transaction):
    n = len(coins)
    dp = [inf] * (transaction + 1)
    for i in range(n):
        if denomination[i] <= transaction:
            for j in range(coins[i]):
                for k in range(transaction, denomination[i], -1):
                    if dp[k - denomination[i]] + 1 < dp[k]:
                        dp[k] = dp[k - denomination[i]] + 1
                dp[denomination[i]] = 1
    return dp

def make_change(coins, denomination, transaction):
    min_cnt = inf
    dp = coin_change_dp(coins, denomination, 200)
    for i in range(transaction, 200):
        cnt = dp[i] + coin_change_greedy(denomination, i - transaction)
        if min_cnt > cnt:
            min_cnt = cnt
    return min_cnt

if __name__ == '__main__':
    denomination = [1, 2, 4, 10, 20, 40]
    for coins, transaction in iter(read_test_case, None):
        print('{:>3}'.format(make_change(coins, denomination, transaction)))
