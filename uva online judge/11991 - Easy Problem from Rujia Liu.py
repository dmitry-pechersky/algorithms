import sys

def read_test_case():
    head = sys.stdin.readline().strip()
    if head == '':
        return None
    n, m = [int(i) for i in  head.split()]
    elements = [int(i) for i in  sys.stdin.readline().split()]
    queries = []
    for i in range(m):
        queries.append([int(i) for i in  sys.stdin.readline().split()])
    return n, m, elements, queries

if __name__ == '__main__':
    for n, m, elements, queries in iter(read_test_case, None):
        adjacency_list = [None] * 1000001
        for i in range(n):
            if  adjacency_list[elements[i]] is None:
                adjacency_list[elements[i]] = []
            adjacency_list[elements[i]].append(i)
        for k, v in queries:
            if len(adjacency_list[v]) < k:
                print(0)
            else:
                print(adjacency_list[v][k - 1] + 1)


        
