import heapq

def read_test_case():
    n = int(input())
    if(n == 0):
        return None
    return [int(i) for i in input().split()]

def add_all(nums):
    heapq.heapify(nums)
    cost = 0 
    while len(nums) > 1:
        a, b = heapq.heappop(nums), heapq.heappop(nums)
        heapq.heappush(nums, a + b)
        cost += a + b
    return cost

if __name__ == '__main__':
    for nums in iter(read_test_case, None):
        print(add_all(nums))