#include <stdio.h>
#include <limits.h>
#define MAX_N 100
#define MAX_M 1000

struct Edge{
    int u;
    int v;
    int weight;
};

int costs[MAX_N];
struct Edge edges[MAX_M];

int bellman_ford(int n, int m, int costs[], struct Edge edges[]){
    int i, j;
    costs[0] = 0;
    for(i = 1; i < n; i++)
        costs[i] = INT_MIN;
    for(i = 0; i < n - 1; i++)
        for(j = 0; j < m; j++)
            if(costs[edges[j].u] != INT_MIN && costs[edges[j].v] <  costs[edges[j].u] + edges[j].weight)
                costs[edges[j].v] = costs[edges[j].u] + edges[j].weight;
    for(j = 0; j < m; j++)
        if(costs[edges[j].u] != INT_MIN && costs[edges[j].v] <  costs[edges[j].u] + edges[j].weight)
            return 1;
    return 0;
}

int main(){
    int i, t, n, m;
    for(scanf("%d", &t); t > 0; t--){
        scanf("%d %d", &n, &m);
        for(i = 0; i < m; i++){
            scanf("%d %d %d", &edges[i].u, &edges[i].v, &edges[i].weight);
            edges[i].u--; edges[i].v--;
        }
        printf(bellman_ford(n, m, costs, edges) ? "Yes\n" : "No\n");
    }
    return 0;
}