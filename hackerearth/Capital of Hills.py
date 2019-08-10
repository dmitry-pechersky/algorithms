if __name__ == '__main__':
    n, hills = int(input()), [int(i) for i in  input().split()]
    cnt, i, stack = 0, 0, []
    while i < len(hills):
        if len(stack) == 0:
            stack.append([hills[i], 1])
            i += 1
        if hills[i] > stack[-1][0]:
            cnt += stack[-1][1]
            stack.pop()
        elif hills[i] == stack[-1][0]:
            cnt += stack[-1][1] + (1 if len(stack) > 1 else 0)
            stack[-1][1] += 1
            i += 1
        else:
            cnt += 1
            stack.append([hills[i], 1])
            i += 1
    print(cnt)

