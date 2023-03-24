def read_test_case():
    n = int(input())
    adj_list = [[] for i in range(n)]
    if n == 0:
        return None
    while True:
        nums = [int(i) - 1 for i in input().split()]
        if nums[0] == -1:
            break
        for u in nums[1:]:
            adj_list[nums[0]].append(u)
            adj_list[u].append(nums[0])
    return adj_list, n

def articulation_points(adj_list, n, visited, depth, low, parents, v, d):
    depth[v], low[v] = d, d
    visited[v] = True
    is_articulation = False
    child_cnt, cnt = 0, 0
    for u in adj_list[v]:
        if not visited[u]:
            child_cnt += 1
            parents[u] = v
            cnt += articulation_points(adj_list, n, visited, depth, low, parents, u, d + 1)
            if low[u] >= depth[v]:
                is_articulation = True  
            low[v] = min(low[v], low[u])
        if parents[v] != u:
            low[v] = min(low[v], depth[u])
    if (parents[v] is not None and is_articulation) or (parents[v] is None and child_cnt > 1):
        cnt += 1
    return cnt
    
if __name__ == '__main__':
    for adj_list, n in iter(read_test_case, None):
        print(articulation_points(adj_list, n, [False] * n, [0] * n, [0] * n, [None] * n, 0, 1))