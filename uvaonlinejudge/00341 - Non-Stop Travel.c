#include <stdio.h>
#include <limits.h>
#define MAX_NODES 10
#define INF INT_MAX

int adj_matrix[MAX_NODES][MAX_NODES], parent[MAX_NODES][MAX_NODES];

int read_test_case(int *n, int adj_matrix[][MAX_NODES], int *start, int *end){
    int i, j, v, weight;
    scanf("%d", n);
    if(*n == 0)
        return 0;
    for(i = 0; i < *n; i++)
        for(j = 0; j < *n; j++)
            adj_matrix[i][j] = INF;       
    for(i = 0; i < *n; i++){
        scanf("%d", &j);
        for(; j > 0; j--){
            scanf("%d %d", &v, &weight);
            if(weight < adj_matrix[i][v - 1]){
                adj_matrix[i][v - 1] = weight;
            }
        }
    }
    scanf("%d %d", start, end);
    (*start)--; (*end)--;
    return 1;
}

void floyd_warshall(int n, int adj_matrix[][MAX_NODES], int parent[][MAX_NODES]){
    int i, j, k;
    for(i = 0; i < n; i++)
        for(j = 0; j < n; j++)
            parent[i][j] = i;
    for(k = 0; k < n; k++){
        for(i = 0; i < n; i++){
            for(j = 0; j < n; j++){
                if(adj_matrix[i][k] != INF &&  adj_matrix[k][j] != INF && adj_matrix[i][j] > adj_matrix[i][k] + adj_matrix[k][j]){
                    adj_matrix[i][j] = adj_matrix[i][k] + adj_matrix[k][j];
                    parent[i][j] = parent[k][j];
                }
            }
        }
    }
}

void print_path(int parent[][MAX_NODES], int start, int end){
    if( start != end)
        print_path(parent, start, parent[start][end]);
    printf(" %d", end + 1);
}

int main(){
    int i = 1, n, start, end ;
    while(read_test_case(&n, adj_matrix, &start, &end)){
        floyd_warshall(n, adj_matrix, parent);
        printf("Case %d: Path =", i);
        print_path(parent, start, end);
        printf("; %d second delay\n", adj_matrix[start][end]);
        i++;
    }
    return 0;
}