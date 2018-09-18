def read_test_cases():
    while True:
        array = []
        while True:
            value = int(input())
            if value == -1:
                break            
            array.append(value)
        if len(array) == 0:
            break
        yield array

def greatest_smaller(array, value):
    a, b = 0, len(array) - 1
    while a != b:
        c = int((a + b) / 2)
        if array[c] >= value:
            a = c + 1
        else:
            b = c
    return b
        
def longest_decreasing_subsequence(array):
    dp = [array[0]]
    for i in range(1, len(array)):
        if dp[-1] >= array[i]:
            dp.append(array[i])
        else:
            dp[greatest_smaller(dp, array[i])] = array[i]
    return len(dp)

if __name__ == '__main__':
    for i, array in enumerate(read_test_cases(), 1):
        if i > 1:
            print()
        print('Test #{}:'.format(i))
        print('  maximum possible interceptions: {}'.format(longest_decreasing_subsequence(array)))
