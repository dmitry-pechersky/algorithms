def max_subarray(seq):
    last_sum, min_sum, max_sub_sum = 0, 0, None
    for a in seq:
        last_sum += a
        if max_sub_sum == None:
            max_sub_sum = a
            min_sum = min(a, min_sum)
        else:
            max_sub_sum = max(last_sum - min_sum, max_sub_sum)
            min_sum = min(last_sum, min_sum)
    return max_sub_sum
 
def max_subsequence(seq):
    max_sub_sum = None
    for a in seq:
        if max_sub_sum == None:
            max_sub_sum = a
        else:
            max_sub_sum = max(max_sub_sum, a, max_sub_sum + a)
    return max_sub_sum

t = int(input())
for i in range(t):
    n = int(input())
    seq = [int(i) for i in input().split()]
    print(max_subarray(seq), max_subsequence(seq))
