import sys

def read_test_case():
    n = int(sys.stdin.readline())
    if n == 0:
        return None
    adjacency_list = [[] for i in range(n + 1)]
    while True:
        line = sys.stdin.readline()
        if line[0] == '0':
            break
        nums = [int(i) for i in line.split()]
        adjacency_list[nums[0]] = nums[1:-1]
    start_vertexes = [int(i) for i in sys.stdin.readline().split()][1:]
    return adjacency_list, start_vertexes

def dfs(adjacency_list, start_vertex):
    n = len(adjacency_list)
    stack, visited = [start_vertex], [False] * n
    while len(stack) > 0:
        vertex = stack.pop()
        for v in adjacency_list[vertex]:
            if not visited[v]:
                visited[v] = True
                stack.append(v)
    return [i for i in range(1, n) if not visited[i]]

if __name__ == '__main__':
    for adjacency_list, start_vertexes in iter(read_test_case, None):
        for start_vertex in start_vertexes:
            res = dfs(adjacency_list, start_vertex)
            print(len(res), *res)

