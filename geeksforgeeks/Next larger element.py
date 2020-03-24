if __name__ == '__main__':
    for t in range(int(input())):
        n, array = int(input()), [int(i) for i in input().split()]
        stack, res = [], [-1] * n
        for i in range(n):
            while stack and array[stack[-1]] < array[i]:
                res[stack.pop()] = array[i]
            stack.append(i)
        print(*res)

                
