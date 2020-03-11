MAX_N =  50

if __name__ == '__main__':
    ways = [0] * (MAX_N + 1)
    ways[0] = 1
    jumps = (1, 2, 3)
    for steps in range(1, MAX_N + 1):
        for jump in jumps:
            if jump <= steps:
                ways[steps] += ways[steps - jump]
    for i in range(int(input())):
        print(ways[int(input())])