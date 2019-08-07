if __name__ == '__main__':
    n, nums = int(input()), [int(i) - 1 for i in input().split()]
    positions = [0] * n
    for i in range(n):
        positions[nums[i]] = i
    swaps = 0
    for i in range(n):
        if i != nums[i]:
            nums[positions[i]] = nums[i]
            positions[nums[i]] = positions[i]
            swaps += 1
    print(swaps)