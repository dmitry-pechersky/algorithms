import sys

def read_matrix():
    head = sys.stdin.readline().strip()
    if head == '':
        return None
    adjacency_list = []
    m, n = [int(i) for i in head.split()]
    for i in range(m):
        columns = [int(j) - 1 for j in input().split()][1:]
        values = [int(j) for j in input().split()]
        adjacency_list.append((columns, values))
    return adjacency_list, m , n

def print_matrix(matrix):
    adjacency_list, m, n = matrix
    print(m, n)
    for columns, values in adjacency_list:
        print(len(columns), *[c + 1 for c in columns], sep=' ')
        print(*values, sep=' ')

def transpose_matrix(matrix):
    adjacency_list, m, n = matrix
    new_adjacency_list = [([],[]) for i in range(n)]
    for i in range(m):
        columns, values = adjacency_list[i]
        for j in range(len(columns)):
            new_columns, new_values = new_adjacency_list[columns[j]]
            new_columns.append(i)
            new_values.append(values[j])
    return new_adjacency_list, n, m

if __name__ == '__main__':
    for matrix in iter(read_matrix, None):
        print_matrix(transpose_matrix(matrix))


