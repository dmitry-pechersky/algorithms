def read_test_case():
    float(input())

def coin_change(coins, amount):
    n = len(coins)
    cache = [0] * (amount + 1)
    for i in range(coins[0], amount + 1, coins[0]):
        cache[i] = 1
    for i in range(1, n):
        for j in range(1, amount + 1):
            if coins[i] < j:
                cache[j] += cache[j - coins[i]]
            elif coins[i] == j: 
                cache[j] += 1
    return cache

if __name__ == '__main__':
    max_amount, coins = 6000, [2000, 1000, 400, 200, 100, 40, 20, 10, 4, 2, 1]
    cache = coin_change(coins, max_amount)
    for amount in iter(lambda: float(input()), 0):
        print("{:>6.2f} {:>16}".format(amount, cache[round(amount * 20) if round(amount * 100) % 5 == 0 else 0]))
