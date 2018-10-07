from math import inf

def minimum_loss(n, prices):
    prices.sort()
    min_loss = inf
    for i in range(n - 1):
        if prices[i + 1][0] - prices[i][0] < min_loss and prices[i][1] > prices[i + 1][1]:
            min_loss = prices[i + 1][0] - prices[i][0]
    return min_loss

if __name__ == '__main__':
    n, prices = int(input()), [(int(price), index) for index, price in enumerate(input().split())]
    print(minimum_loss(n, prices))
