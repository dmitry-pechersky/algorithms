def minimum_unfairness_dp(n, k, packets):
    packets.sort()
    total, unfairness, unfairness_i = packets[0], 0, 0
    for i in range(1, k):
        unfairness_i = unfairness_i + (packets[i] - packets[i - 1]) * i
        unfairness += unfairness_i
        total += packets[i]
    min_unfairness = unfairness
    for i in range(n - k):
        total -= packets[i]
        unfairness -= total - packets[i] * (k - 1)
        unfairness += packets[i + k] * (k - 1) - total
        total += packets[i + k]
        if unfairness < min_unfairness:
            min_unfairness = unfairness
    return min_unfairness

if __name__ == '__main__':
    n, k = int(input()), int(input())
    packets = [int(input()) for i in range(n)]
    print(minimum_unfairness_dp(n, k, packets))
