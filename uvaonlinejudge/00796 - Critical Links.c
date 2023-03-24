#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#define MAX_NODES 200
#define MIN(a,b) (((a)<(b))?(a):(b))
#define MAX(a,b) (((a)>(b))?(a):(b))

int n;
int adj_list[MAX_NODES][MAX_NODES + 1], visited[MAX_NODES], low[MAX_NODES], depth[MAX_NODES], parent[MAX_NODES];

struct bridge{
    int v;
    int u;
};
struct bridge bridges[MAX_NODES * MAX_NODES];
int bridge_cnt;

int compare_bridges(const void *a, const void *b){
    struct bridge *bridge_a = (struct bridge *) a, *bridge_b = (struct bridge *) b;
    if(bridge_a->v == bridge_b->v){
        if(bridge_a->u == bridge_b->u){
            return 0;
        }else{
            return bridge_a->u < bridge_b->u ? -1 : 1;
        }
    }else{
        return bridge_a->v < bridge_b->v ? -1 : 1;
    }
}

int read_test_case(){
    int i, j, v, cnt;
    if(scanf("%d", &n) == EOF)
        return 0;
    for(i = 0; i < n; i++){
        scanf("%d (%d)", &v, &cnt);
        adj_list[v][0] = cnt;
        for(j = 1; j <= cnt; j++){
            scanf("%d", &adj_list[v][j]);
        }
    }
    return 1;
}

void articulation_bridges(int v, int d){
    int i, u;
    visited[v] = 1;
    low[v] = depth[v] = d;
    for(i = 1; i <= adj_list[v][0]; i++){
        u = adj_list[v][i];
        if(!visited[u]){
            parent[u] = v;
            articulation_bridges(u, d + 1);
            if(low[u] > depth[v]){
                bridges[bridge_cnt].v = MIN(u, v);
                bridges[bridge_cnt].u = MAX(u, v);
                bridge_cnt++;
            }
            low[v] = MIN(low[v], low[u]);
        }else if(parent[v] != u){
            low[v] = MIN(low[v], depth[u]);
        }
    }
}

int main(){
    int i;
    while(read_test_case()){
        memset(visited, 0, sizeof(visited));
        for(i = 0; i < n; i++){
            parent[i] = -1;
        }
        bridge_cnt = 0;
        for(i = 0; i < n; i++){
            if(!visited[i])
                articulation_bridges(i, 1);
        }
        qsort(bridges, bridge_cnt, sizeof(struct bridge),compare_bridges);
        printf("%d critical links\n", bridge_cnt);
        for(i = 0; i < bridge_cnt; i++){
            printf("%d - %d\n", bridges[i].v, bridges[i].u);
        }
        printf("\n");
    }
    return 0;
}