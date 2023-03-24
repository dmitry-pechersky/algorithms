import sys

def read_test_case():
    junctions = [int(i) for i in sys.stdin.readline().split()]
    if len(junctions) == 0:
        return None
    r = int(sys.stdin.readline())
    edge_list = []
    for i in range(r):
        x, y = (int(i) for i in sys.stdin.readline().split())
        edge_list.append((x, y, (junctions[y] - junctions[x]) ** 3))
    q = int(sys.stdin.readline())
    queries = []
    for i in range(q):
        queries.append(int(sys.stdin.readline()))
    return junctions[0], edge_list, queries

def bellman_ford(n, edge_list):
    distance = [None] * (n + 1)
    is_negative_cycle = [False] * (n + 1)
    distance[1] = 0
    for i in range(2 * n - 1):
        changed = False
        for x, y, weight in edge_list:
            if distance[x] is not None and (distance[y] is None or distance[x] + weight < distance[y]):
                distance[y] = distance[x] + weight
                changed = True
                if i >= n - 1:
                    is_negative_cycle[y] = True
        if not changed:
            break
    return distance, is_negative_cycle

if __name__ == '__main__':
    for i, (n, edge_list, queries) in enumerate(iter(read_test_case, None)):
        print('Set #{}'.format(i + 1))
        if n > 0:
            distances, is_negative_cycle = bellman_ford(n, edge_list)
            for x in queries:
                print('?' if x <= 0 or x > n or distances[x] is None or distances[x] < 3 or is_negative_cycle[x] else distances[x])