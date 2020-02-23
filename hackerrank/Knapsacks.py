INF = float('Inf')

def dp_knapsack_big_weights(n, c, costs, weights):
    max_cost = 0
    total_cost = sum(costs)
    dp_cur = [INF] * (total_cost + 1)
    for i in range(n):
        dp_prev = dp_cur
        dp_cur = [INF] * (total_cost + 1)
        for cost in range(1, total_cost + 1):
            if costs[i] >= cost:
                dp_cur[cost] = weights[i] if dp_prev[cost] > weights[i] else dp_prev[cost]
            else:
                dp_cur[cost] =  dp_prev[cost - costs[i]] + weights[i] if  dp_prev[cost - costs[i]] + weights[i] < dp_prev[cost] else dp_prev[cost]
            if dp_cur[cost] <= c and cost > max_cost:
                max_cost = cost
            if dp_cur[cost] > c:
                break
    return max_cost

if __name__ == '__main__':
    n, c = (int(i) for i in input().split())
    costs = [int(i) for i in input().split()]
    weights = [int(i) for i in input().split()]
    print(dp_knapsack_big_weights(n, c, costs, weights))