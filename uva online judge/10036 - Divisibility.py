def read_test_case():
    n, k = (int(i) for i in input().split())
    return [abs(int(i)) % k for i in input().split()], k

def divisibility_dp(nums, k):
    n = len(nums)
    current = [False] * k
    current[0] = True
    for i in nums:
        previous, current = current, [False] * k
        for j in range(0, k):
            if previous[j]:
                current[abs(j + i) % k] = True
                current[abs(j - i) % k] = True
    return current[0]

if __name__ == '__main__':
    for i in range(int(input())):
        print('Divisible' if divisibility_dp(*read_test_case()) else 'Not divisible')
