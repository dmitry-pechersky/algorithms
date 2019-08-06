def read_test_cases():
    for i in range(int(input())):
        input()
        yield [int(j) for j in input().split()]

def count_bribes(state):
    bribes = 0
    for i in range(len(state)):
        if state[i] - i > 2 + 1:
            return None
        j = i
        while j > 0 and state[j] < state[j - 1]:
            state[j - 1], state[j] = state[j], state[j - 1]
            j -= 1 
            bribes += 1
    return bribes

if __name__ == '__main__':
    for state in read_test_cases():
        bribes = count_bribes(state)
        print('Too chaotic' if bribes is None else bribes) 
   