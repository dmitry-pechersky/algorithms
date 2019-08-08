if __name__ == '__main__':
    n, r = map(int, input().split())
    dic_1, dic_2, cnt = {}, {}, 0
    for num in map(int, input().split()):
        if num % r == 0:
            cnt += dic_2.get(num // r, 0) 
            dic_2[num] =  dic_2.get(num, 0) + dic_1.get(num // r, 0)
        dic_1[num] = dic_1.get(num, 0) + 1
    print(cnt)
