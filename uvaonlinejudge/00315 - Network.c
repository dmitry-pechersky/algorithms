#include <stdio.h>
#include <string.h>
#define MAX_NODES 99
#define MIN(x, y) (((x) < (y)) ? (x) : (y))

int n;
int adj_list[MAX_NODES + 1][MAX_NODES * 2 + 1];
int visited[MAX_NODES + 1];
int depth[MAX_NODES + 1];
int low[MAX_NODES + 1];
int parents[MAX_NODES + 1];

int read_test_case(){
    int i, v, u;
    char c;    
    scanf("%d", &n);
    if(n == 0)
        return 0;
    for(i = 1; i <= n; i++)
        adj_list[i][0] = 0;
    while(scanf("%d", &v) && v > 0){
        c = '0';
        while(c != '\n'){
            scanf("%d%c", &u, &c);
            adj_list[v][++adj_list[v][0]] = u;
            adj_list[u][++adj_list[u][0]] = v;
        }
    }
    return 1;
}

int articulation_points(int v, int d){
    int i, u, is_articulation = 0, cnt = 0, child_cnt = 0;
    visited[v] = 1;
    depth[v] = low[v] = d;
    for(i = 1; i <= adj_list[v][0]; i++){
        u = adj_list[v][i];
        if(!visited[u]){
            child_cnt++;
            parents[u] = v;
            cnt += articulation_points(u, d + 1);
            if(low[u] >= depth[v])
                is_articulation = 1;
            low[v] = MIN(low[v], low[u]);
        }else if(parents[v] != u){
            low[v] = MIN(depth[u], low[v]);
        }   
    }
    if((parents[v] != 0 && is_articulation) || (parents[v] == 0 && child_cnt > 1))
        cnt++;
    return cnt;
}

int main(){
    while(read_test_case()){
        memset(visited, 0, sizeof(visited));
        memset(parents, 0, sizeof(parents));
        printf("%d\n", articulation_points(1, 1));
    }
    return 0;
}