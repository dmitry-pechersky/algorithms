if __name__ == '__main__':
    for t in range(int(input())):
        n, heights = int(input()), [int(i) for i in input().split()]
        i, stack, ranges = 0, [], [1] * len(heights)
        while i < len(heights):
            if len(stack) == 0:
                stack.append(i)
                i += 1
            elif heights[stack[-1]] > heights[i]:
                stack.append(i)
                i += 1
            else:
                ranges[i] += ranges[stack.pop()]
        print(*ranges)
