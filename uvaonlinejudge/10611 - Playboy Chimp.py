import math

def shortest(heights, i, j, h):
    k = (i + j) // 2
    if i == j:
        return i
    if heights[k] > h:
        return shortest(heights, i, k, h)
    return shortest(heights, k + 1, j, h)

def tallest(heights, i, j, h):
    k = math.ceil((i + j) / 2)
    if i == j:
        return i
    if heights[k] >= h:
        return tallest(heights, i, k - 1, h)
    return tallest(heights, k, j, h)

if __name__ == '__main__':
    n = int(input())
    heights = [int(i) for i in input().split()]
    q = int(input())
    for h in (int(i) for i in input().split()):
        tallest_i = tallest(heights, 0, n - 1, h)
        shortest_i = shortest(heights, 0, n - 1, h)
        print(heights[tallest_i] if heights[tallest_i] < h  else 'X', heights[shortest_i] if heights[shortest_i] > h else 'X')
