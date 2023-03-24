import math 
def read_test_case():
    n = int(input())
    adj_matrix = [[math.inf] * n for i in range(n)]
    for i in range(1,n):
        weights = input().split()
        for j in range(i):
            if weights[j] != 'x':
                adj_matrix[i][j] = int(weights[j])
                adj_matrix[j][i] = int(weights[j])
    return n, adj_matrix

def floyd_warshall(n, adj_matrix):
    for k in range(n):
        for i in range(n):
            for j in range(n):
                adj_matrix[i][j] = min(adj_matrix[i][j], adj_matrix[i][k] + adj_matrix[k][j])
    return adj_matrix

if __name__ == '__main__':
    n, adj_matrix = read_test_case()
    floyd_warshall(n, adj_matrix)
    print(max(adj_matrix[i][0] for i in range(1,n)))