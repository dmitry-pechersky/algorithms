def read_train():
    return int(input())

def read_permutation():
    line = input()
    if line[0] == '0':
        return None
    return [int(i) for i in line.split()]

def is_possible(n, permutation):
    i, stack = 1,  []
    for j in permutation:
        while True:
            if len(stack) > 0 and stack[-1] == j:
                stack.pop()
                break
            if i <= n:
                stack.append(i)
                i += 1
            else:
                return False
    return True

if __name__ == '__main__':
    for n in iter(read_train, 0):
        for permutation in iter(read_permutation, None):
            print('Yes' if is_possible(n, permutation) else 'No')
        print()
