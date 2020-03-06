MAX_N = 10000

if __name__ == '__main__':
    operations = [0] * (MAX_N + 1)
    for i in range(1, MAX_N + 1):
        operations[i] = operations[i - 1] + 1
        if i % 2 == 0 and operations[i // 2] + 1 < operations[i]:
            operations[i] = operations[i // 2] + 1
    for i in range(int(input())):
        print(operations[int(input())])
