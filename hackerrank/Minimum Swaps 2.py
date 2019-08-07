if __name__ == '__main__':
    n, nums = int(input()), [int(i) - 1 for i in input().split()]
    swaps = 0
    for i in range(n):
        while nums[i] != i:
            tmp = nums[i]
            nums[i], nums[tmp] = nums[tmp], tmp
            swaps += 1
    print(swaps)
