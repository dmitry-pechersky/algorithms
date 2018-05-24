import math

def chief_hopper(n, heights):
    energy = 0
    for i in range(n-1, -1, -1):
            energy = math.ceil((energy + heights[i]) / 2)
    return energy

n = int(input())
heights = [int(i) for i in input().split()]
print(chief_hopper(n, heights))
