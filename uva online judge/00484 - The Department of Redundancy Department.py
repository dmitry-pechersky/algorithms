import sys

nums, dic = [], {}
for line in sys.stdin:
    for num in (int(i) for i in line.split()):
        if num not in dic:
            nums.append(num)
        dic[num] = dic.get(num, 0) + 1
for num in nums:
    print(num, dic[num])
