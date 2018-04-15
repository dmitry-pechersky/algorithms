import heapq

def add_all(nums):
    queue = []
    cost = 0 
    for num in nums:
        heapq.heappush(queue, num)
    while len(queue) > 1:
        a, b = heapq.heappop(queue), heapq.heappop(queue)
        heapq.heappush(queue, a + b)
        cost += a + b
    return cost

if __name__ == '__main__':
    while True:
        n = int(input())
        if n == 0:
            break
        nums = [int(i) for i in input().split()]
        print(add_all(nums))