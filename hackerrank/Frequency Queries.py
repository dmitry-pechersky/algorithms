import sys

if __name__ == '__main__':
    max_queries = 1000000
    dic, frequencies =  {}, [0] * (max_queries + 1)
    sys.stdin.readline()
    for line in sys.stdin.readlines():
        query, value = (int(i) for i in line.split())
        if query == 1:
            cnt = dic.get(value, 0)
            dic[value] = cnt + 1
            if cnt > 0:
                frequencies[cnt] -= 1
            frequencies[cnt + 1] += 1
        elif query == 2:
            cnt = dic.get(value, 0)
            if cnt > 0:
                dic[value] = cnt - 1
                frequencies[cnt] -= 1
                if cnt - 1 > 0:
                    frequencies[cnt - 1] += 1
        else:
            print(0 if value > max_queries or frequencies[value] == 0 else 1)
