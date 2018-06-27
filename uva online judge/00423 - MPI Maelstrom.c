#include <stdio.h>
#include <limits.h>
#include <stdlib.h>
#define MAX_NODES 100
#define INF INT_MAX


int adj_matrix[MAX_NODES][MAX_NODES];

void read_test_case(int *n, int adj_matrix[][MAX_NODES]){
    int i, j;
    char buffer[1024];
    scanf("%d", n);
    for(i = 0; i < *n ; i++)
        for(j = 0; j < *n; j++)
            adj_matrix[i][j] = INF;
    for(i = 1; i < *n; i++){
        for(j = 0; j < i; j++){
            scanf("%s", buffer);
            if(buffer[0] != 'x'){
                adj_matrix[i][j] = atoi(buffer);
                adj_matrix[j][i] = adj_matrix[i][j];
            }
        }
    }
}

void floyd_warshall(int n, int adj_matrix[][MAX_NODES]){
    int i, j, k;
    for(k = 0; k < n; k++)
        for(i = 0; i < n; i++)
            for(j = 0; j < n; j++)
                if(adj_matrix[i][k] != INF && adj_matrix[k][j] != INF && (adj_matrix[i][k] + adj_matrix[k][j]) <  adj_matrix[i][j])
                    adj_matrix[i][j] = adj_matrix[i][k] + adj_matrix[k][j];
}

int main(){
    int n, i, max_value = 0;
    read_test_case(&n,adj_matrix);
    floyd_warshall(n, adj_matrix);
    for(i = 1; i < n; i++)
        max_value =  max_value < adj_matrix[i][0] ? adj_matrix[i][0] : max_value;
    printf("%d\n", max_value);
    return 0;
}