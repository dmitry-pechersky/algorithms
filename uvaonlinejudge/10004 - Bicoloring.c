#include <stdio.h>
#define MAX_NODES 199

int read_test_case(int adj_list[MAX_NODES][MAX_NODES + 1], int *n){
    int i, l, u, v;
    scanf("%d", n);
    if(!*n)
        return 0;
    for(i = 0; i < *n; i++){
        adj_list[i][0] = 0;
    }
    scanf("%d", &l);
    for(i = 0; i < l; i++){
        scanf("%d %d", &u, &v);
        adj_list[u][++adj_list[u][0]] = v;
        adj_list[v][++adj_list[v][0]] = u;        
    }
    return 1;
}

int is_bicolored(int adj_list[MAX_NODES][MAX_NODES + 1], int n){
    int i, u, v, colors[MAX_NODES] = {1};
    int stack[MAX_NODES], stack_size = 1;
    stack[0] = 0;
    while(stack_size > 0){
        v = stack[--stack_size];
        for(i = 1; i <= adj_list[v][0]; i++){
            u = adj_list[v][i];
            if(colors[u] == 0){
                colors[u] = 3 - colors[v];
                stack[stack_size++] = u;
            }else if(colors[u] == colors[v]){
                return 0;
            }
        }
    }
    return 1;
}

int main(){
    int n, adj_list[MAX_NODES][MAX_NODES + 1];
    while(read_test_case(adj_list, &n)){
        printf(is_bicolored(adj_list, n) ? "BICOLORABLE.\n" : "NOT BICOLORABLE.\n");
    }
    return 0;
}