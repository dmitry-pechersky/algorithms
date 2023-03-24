def read_matrix():
    n, m = map(int, input().split())
    matrix = []
    for i in range(n):
        matrix.append([int(j) for j in input().split()])
    return n,m, matrix

def is_undirected_graph(n, m, matrix):
    adjacency_matrix = [[0] * n for i in range(n)]
    for i in range(m):
        a, b = None, None
        for j in range(n):
            if matrix[j][i] == 1:
                if a is None:
                    a = j
                elif b is None:
                    b = j
                else:
                    return False
        if b is not None and adjacency_matrix[a][b] == 0:
            adjacency_matrix[a][b] = 1
            adjacency_matrix[b][a] = 1
        else:
            return False            
    return True

if __name__ == '__main__':
    t = int(input())
    for i in range(t):
        print('Yes' if is_undirected_graph(*read_matrix()) else 'No')
