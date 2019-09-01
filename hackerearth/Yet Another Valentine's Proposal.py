if __name__ == '__main__':
    n, q = map(int, input().split())
    cnts = [[0] * 26]
    for ch in input().strip():        
        cnts.append(cnts[-1][:])
        cnts[-1][ord(ch) - 97] += 1
    for l,r in (map(int, input().split()) for t in range(q)):
        string = [chr((cnts[r][i] - cnts[l - 1][i]) % 26  + 97)  for i in range(26)]
        res_i = None
        for i in range(1, 26):
            if string[0:26 - i] == string[i:]:
                res_i = i
                break
        if res_i == None:
            print('None')
        else:
            print(*string[res_i:], sep='')
        
