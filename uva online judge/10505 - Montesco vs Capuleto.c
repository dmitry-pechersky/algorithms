#include <stdio.h>
#include <string.h>
#define MAX_NODES 200
#define MAX(a,b) (((a)>(b))?(a):(b))

void read_test_case(int adj_list[MAX_NODES][MAX_NODES * 2 + 1], int *n){
    int u, v, i;
    scanf("%d", n);
    for(u = 0; u < *n; u++)
        adj_list[u][0] = 0;
    for(u = 0; u < *n; u++){
        for(scanf("%d", &i); i > 0; i--){
            scanf("%d", &v);
            if(--v < *n){
                adj_list[u][++adj_list[u][0]] = v;
                adj_list[v][++adj_list[v][0]] = u;
            }
        }
    }
}

int bicolor(int adj_list[MAX_NODES][MAX_NODES * 2 + 1], int colors[MAX_NODES], int n, int start){
    int stack[MAX_NODES], stack_size = 1;
    int u, v, i, is_bipartite = 1, cnts[3] = {0, 1, 0};
    stack[0] = start;
    colors[start] = 1;
    while(stack_size > 0){
        u = stack[--stack_size];
        for(i = 1; i <= adj_list[u][0]; i++){
            v = adj_list[u][i];
            if(colors[v] == 0){
                colors[v] = 3 - colors[u];
                stack[stack_size++] = v;
                cnts[colors[v]]++;
            }else if(colors[v] == colors[u]){
                is_bipartite = 0;
            }
        }
    }
    return MAX(cnts[1], cnts[2]) * is_bipartite;
}

int main(){
    int t, n, i, cnt;
    int adj_list[MAX_NODES][MAX_NODES * 2 + 1], colors[MAX_NODES];
    for(scanf("%d", &t); t > 0; t--){
        read_test_case(adj_list, &n);
        memset(colors, 0, sizeof(int) * n);
        cnt = 0;
        for(i = 0; i < n; i++){
            if(colors[i] == 0){
                cnt += bicolor(adj_list, colors, n, i);
            }
        }
        printf("%d\n", cnt);
    }
    return 0;
}