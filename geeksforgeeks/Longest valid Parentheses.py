def longest_valid_parentheses(string):
    n = len(string)
    closed, stack, length, max_length = [False] * n, [], 0, 0
    for i in range(n):
        if string[i] == '(':
            stack.append(i)
        elif stack:
            j = stack.pop()
            closed[i], closed[j] = True, True
    for i in range(n):
        if closed[i]:
            length += 1
            max_length = length if length > max_length else max_length
        else:
            length = 0
    return max_length

if __name__ == '__main__':
    for i in range(int(input())):
        print(longest_valid_parentheses(input().strip()))
