def read_test_case():
    return [ord(i) - 65 for i in input()]

def find(sequence):
    trails = [0] * 26
    stack = [sequence[0]]
    i = 1
    while len(stack) > 0:
        if stack[-1] == sequence[i]:
            stack.pop()
        else:
            trails[stack[-1]] += 1
            trails[sequence[i]] += 1
            stack.append(sequence[i])
        i += 1
    return trails

if __name__ == '__main__':
    t = int(input())
    for i in range(1, t + 1):
        print("Case {}".format(i))
        trails = find(read_test_case())
        for j in range(26):
            if trails[j] > 0:
                print("{} = {}".format(chr(j + 65), trails[j]))
