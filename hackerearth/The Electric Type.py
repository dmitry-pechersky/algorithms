if __name__ == '__main__':
    n = int(input())
    nums = list(map(int, input().split()))
    uniq, num_set = [0] * n, set()
    for i in range(n - 1, 0, -1):
        num_set.add(nums[i])
        uniq[i] = len(num_set)
    cnt, num_set = 0, set()
    for i in range(n - 1):
        if nums[i] not in num_set:
            num_set.add(nums[i])
            cnt += uniq[i + 1]
    print(cnt)