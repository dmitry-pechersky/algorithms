import heapq
import math

def read_test_case():
    input()
    n = int(input())
    vertex_list = [[float(j) for j in input().split()] for i in range(n)]
    return vertex_list

def distance(vertex_list, a, b):
    return math.hypot((vertex_list[a][0] - vertex_list[b][0]), (vertex_list[a][1] - vertex_list[b][1]))

def prim(vertex_list):
    n = len(vertex_list)
    cost, cnt, queue, in_mst = 0, 0, [], [False] * n
    in_mst[0] = True
    for u in range(1, n):
        heapq.heappush(queue, (distance(vertex_list, 0, u), u))
    while len(queue) > 0 and cnt < n - 1:
        w, v = heapq.heappop(queue)
        if not in_mst[v]:
            cnt, cost = cnt + 1, cost + w
            in_mst[v] = True
            for u in range(n):
                if not in_mst[u]:
                    heapq.heappush(queue, (distance(vertex_list, v, u), u))
    return cost            

if __name__ == '__main__':
    for i in range(int(input())):
        if i > 0:
            print()
        print("{:.2f}".format(prim(read_test_case())))

        