n, k = int(input()), int(input())
numbers = []
for i in range(n):
    numbers.append(int(input()))
numbers = sorted(numbers)
unfairness = 1000000000
for i in range(n - k + 1):
    dif = numbers[i + k - 1] - numbers[i]
    if dif < unfairness:
        unfairness = dif
print(unfairness)

