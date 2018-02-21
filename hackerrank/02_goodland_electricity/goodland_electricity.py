def count_towers(n, k, towers):
    last_enabled, last_tower, enabled_cnt  = - k, -k, 0
    for i in range(n):
        if towers[i] == 1:
            last_tower = i
        if i - last_enabled > 2 * (k - 1):
            if last_tower > last_enabled:
                last_enabled = last_tower
                enabled_cnt += 1
            else:
                return -1
    if n - 1  - last_enabled > k - 1:
        if n - 1 - last_tower > k -1:
            return -1
        else:
            enabled_cnt += 1
    return enabled_cnt

n, k = [int(i) for i in input().split()]
towers = [int(i) for i in input().split()]
print(count_towers(n, k, towers))

