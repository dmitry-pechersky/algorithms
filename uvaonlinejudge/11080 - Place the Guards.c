#include <stdio.h>
#include <string.h>
#define MAX_NODES 200
#define MIN(a,b) (((a)<(b))?(a):(b))

void read_test_case(int adj_list[MAX_NODES][MAX_NODES + 1], int *n){
    int e, i, u, v;
    scanf("%d %d", n, &e);
    for(i = 0; i < *n; i++)
        adj_list[i][0] = 0;
    for(i = 0; i < e; i++){
        scanf("%d %d", &u, &v);
        adj_list[u][++adj_list[u][0]] = v;
        adj_list[v][++adj_list[v][0]] = u;
    }
}

int bicolor(int adj_list[MAX_NODES][MAX_NODES + 1], int colors[MAX_NODES], int n, int start){
    int stack[MAX_NODES], stack_size = 1;
    int u, v, i, cnts[3] = {0, 1, 0};
    colors[start] = 1;
    stack[0] = start;
    while(stack_size > 0){
        u = stack[--stack_size];
        for(i = 1; i <= adj_list[u][0]; i++){
            v = adj_list[u][i];
            if(colors[v] == 0){
                colors[v] = 3 - colors[u];
                stack[stack_size++] = v;
                cnts[colors[v]]++;
            }else if(colors[v] == colors[u]){
                return -1;
            }
        }
    }
    return MIN(cnts[1], cnts[2]);
}

int main(){
    int t, n, i, min_cnt, cnt;
    int adj_list[MAX_NODES][MAX_NODES + 1], colors[MAX_NODES];
    for(scanf("%d", &t); t > 0; t--){
        read_test_case(adj_list, &n);
        memset(colors, 0, sizeof(int) * n);
        cnt = 0;
        for(i = 0; i < n; i++){
            if(colors[i] == 0){
                min_cnt = bicolor(adj_list, colors, n, i);
                if(min_cnt == -1){
                    cnt = -1;
                    break;
                }
                cnt += min_cnt > 0 ? min_cnt : 1;
            }
        }
        printf("%d\n", cnt);
    }
    return 0;
}