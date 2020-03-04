def prize_money(n, coins):
    max_coin = max(coins)
    values = [False] * (2 * max_coin)
    values[0] = True
    for value in range(1, 2 * max_coin):
        for coin in coins:
            if value - coin >= 0 and values[value - coin]:
                values[value] = True
                break
        if value > max_coin and not values[value]:
            return value
    return None

if __name__ == '__main__':
    n = int(input())
    coins = [int(i) for i in input().split()]
    prize = prize_money(n, coins)
    print(prize if prize is not None else 'Fake Offer!')
    
