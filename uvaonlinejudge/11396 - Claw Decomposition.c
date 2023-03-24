#include <stdio.h>
#include <string.h>
#define MAX_NODES 300

int read_test_case(int adj_list[MAX_NODES][MAX_NODES + 1], int *n){
    int u, v;
    scanf("%d", n);
    if(*n == 0)
        return 0;
    for(u = 0; u < *n; u++){
        adj_list[u][0] = 0;
    }
    while(scanf("%d %d", &u, &v) && u != 0){
        adj_list[u - 1][++adj_list[u - 1][0]] = v - 1;
        adj_list[v - 1][++adj_list[v - 1][0]] = u - 1;
    }
    return 1;
}

int is_bipartite(int adj_list[MAX_NODES][MAX_NODES + 1], int colors[MAX_NODES], int n, int start){
    int i, u, v;
    int stack[MAX_NODES], stack_size = 1;
    stack[0] = start;
    colors[start] = 1;
    while(stack_size > 0){
        u = stack[--stack_size];
        for(i = 1; i <= adj_list[u][0]; i++){
            v = adj_list[u][i];
            if(colors[v] == 0){
                colors[v] = 3 - colors[u];
                stack[stack_size++] = v;
            }else if(colors[v] == colors[u]){
                return 0;
            }
        }
    }
    return 1;
}

int main(){
    int i, res, n, adj_list[MAX_NODES][MAX_NODES + 1], colors[MAX_NODES];
    while(read_test_case(adj_list, &n)){
        memset(colors, 0, sizeof(int) * n);
        res = 1;
        for(i = 0; i < n; i++){
            if(colors[i] == 0 && ! is_bipartite(adj_list, colors, n, i)){
                res = 0;
                break;
            }
        }
        printf(res ? "YES\n" : "NO\n");
    }
    return 0;
}