import math

def read_test_case():
    n = int(input())
    if n == 0:
        return None
    adj_matrix = [[math.inf] * n for i in range(n)]
    for i in range(0, n):
        nums = [int(j) for j in input().split()]
        for j in range(nums[0]):
            adj_matrix[i][nums[2 * j + 1] - 1] = min(nums[2 * j + 2], adj_matrix[i][nums[2 * j + 1] - 1])
    start, end = (int(i) - 1 for i in input().split())
    input()
    return n, adj_matrix, start, end

def floyd_warshall(n, adj_matrix):
    parent = [[i] * n for i in range(n)]
    for k in range(n):
        for i in range(n):
            for j in range(n):
                if adj_matrix[i][k] + adj_matrix[k][j] < adj_matrix[i][j]:
                    adj_matrix[i][j] = adj_matrix[i][k] + adj_matrix[k][j]
                    parent[i][j] = parent[k][j]
    return  adj_matrix, parent

def print_path(parent, start, end):
    if start != end:
        print_path(parent, start, parent[start][end])
    print(' {}'.format(end + 1), end='')
    
if __name__ == '__main__':
    for i, (n, adj_matrix, start, end) in enumerate(iter(read_test_case, None), start=1):
        adj_matrix, parent = floyd_warshall(n, adj_matrix)        
        print('Case {}: Path ='.format(i), end='') 
        print_path(parent, start, end)
        print('; {} second delay'.format(adj_matrix[start][end]))