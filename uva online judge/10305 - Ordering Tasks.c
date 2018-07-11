#include <stdio.h>
#define MAX_NODES 100

int adj_list[MAX_NODES][MAX_NODES + 1], visited[MAX_NODES], order[MAX_NODES];

int read_test_case(int *n, int adj_list[][MAX_NODES + 1]){
    int m, i, v, u;
    scanf("%d %d", n, &m);
    if(*n == 0)
        return 0;
    for(i = 0; i < *n; i++)
        adj_list[i][0] = 0;
    for(i = 0; i < m; i++){
        scanf("%d %d", &v, &u);
        adj_list[v - 1][++adj_list[v - 1][0]] = u - 1;
    }
    return 1;
}

void dfs(int n, int adj_list[][MAX_NODES + 1], int v, int visited[], int order[], int *idx){
    int i;
    visited[v] = 1;
    for(i = 1; i <= adj_list[v][0]; i++)
        if(!visited[adj_list[v][i]])
            dfs(n, adj_list, adj_list[v][i], visited, order, idx);
    order[(*idx)++] = v;
}

void toposort(int n, int adj_list[][MAX_NODES + 1], int visited[], int order[]){
    int v, idx = 0;
    for(v = 0; v < n; v++)
        visited[v] = 0;
    for(v = 0; v < n; v++)
        if(!visited[v])
            dfs(n, adj_list, v, visited, order, &idx);
}

int main(){
    int n, i;
    while(read_test_case(&n, adj_list)){
        toposort(n, adj_list, visited, order);
        for(i = n - 1; i >= 0; i--)
            printf("%d ", order[i] + 1);
        printf("\n");
    }
    
    return 0;
}