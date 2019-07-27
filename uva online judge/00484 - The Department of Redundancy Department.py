nums, dic = [], {}
for num in (int(i) for i in input().split()):
    if num not in dic:
        nums.append(num)
    dic[num] = dic.get(num, 0) + 1
for num in nums:
    print(num, dic[num])
