def is_possible(string, dp):
    if len(string) == 0:
        return True
    if string not in dp:
        index, cnt = 0, 1
        for i in range(1, len(string)):
            if string[index] == string[i]:
                cnt += 1
            else:
                if cnt > 1:
                    if is_possible(string[:index] + string[index + cnt:], dp):
                        return True
                index, cnt = i, 1
        if cnt > 1:
            if is_possible(string[:index] + string[index + cnt:], dp):
                return True
        dp.add(string)
    return False

if __name__ == '__main__':
    for i in range(int(input())):
        print(1 if is_possible(input().strip(), set()) else 0)
