from math import hypot, inf

def read_test_case():
    n = int(input())
    if n == 0:
        return None
    computers = []
    for i in range(n):
         computers.append([int(j) for j in input().split()])
    return n, computers

def build_adj_matrix(n, vertices):
    adj_matrix = [[0] * n for i in range(n)]
    for i in range(n):
        for j in range(i + 1, n):
            adj_matrix[i][j] = hypot(vertices[i][0] - vertices[j][0], vertices[i][1] - vertices[j][1]) + 16
            adj_matrix[j][i] = adj_matrix[i][j]
    return adj_matrix

def traveling_salesman(n, adj_matrix):
    tsp = [[0] * 2 ** n  for i in range(n)]
    for s in  range(2 ** n):
        for i in range(n):
            if 1 << i == s:
                tsp[i][s] = 0
            elif (1 << i & s) == 0:
                tsp[i][s] = inf
            else:
                s_minus_i = s ^ 1 << i
                tsp[i][s] = inf
                for j in range(n):
                    if tsp[j][s_minus_i] + adj_matrix[i][j] < tsp[i][s]:
                        tsp[i][s] = tsp[j][s_minus_i] + adj_matrix[i][j]
    i, s = 0, 2 ** n - 1
    for j in range(1, n):
        if tsp[i][s] > tsp[j][s]:
            i = j
    path = []
    for k in range(n - 1):
        path.append(i)
        s_minus_i = s ^ (1 << i)
        for j in range(0, n):
            if tsp[j][s_minus_i] + adj_matrix[i][j] == tsp[i][s]:
                i = j
                break
        s = s_minus_i
    path.append(i)
    return path

if __name__ == '__main__':
    for i, (n, computers) in enumerate(iter(read_test_case, None), 1):
        adj_matrix = build_adj_matrix(n, computers)
        print('**********************************************************')
        print('Network #{}'.format(i))
        path = traveling_salesman(n, adj_matrix)
        total_feet = 0
        for j in range(n - 1):
            print('Cable requirement to connect ({},{}) to ({},{}) is {:.2f} feet.'.format(
                *computers[path[j]], 
                *computers[path[j + 1]], 
                adj_matrix[path[j]][path[j + 1]]))
            total_feet += adj_matrix[path[j]][path[j + 1]]
        print('Number of feet of cable required is {:.2f}.'.format(total_feet))
