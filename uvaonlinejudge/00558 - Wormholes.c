#include <stdio.h>
#define MAX_NODES 1000
#define MAX_EDGES 2000
#define INF 2000001

struct Edge{
    int x;
    int y;
    int weight;
};

int distances[MAX_NODES];

struct Edge edge_list[MAX_EDGES];

void read_test_case(int *n, int *m, struct Edge edge_list[]){
    int i;
    scanf("%d %d", n, m);
    for(i = 0; i < *m; i++)
        scanf("%d %d %d", &edge_list[i].x, &edge_list[i].y, &edge_list[i].weight);
}

int bellman_ford(int n, int m, struct Edge edge_list[], int distances[]){
    int i, j, changed;
    for(i = 1; i < n; i++)
        distances[i] = INF;
    distances[0] = 0;
    for(i = 0; i < n - 1; i++){
        changed = 0;
        for(j = 0; j < m; j++){
            if(distances[edge_list[j].x] +  edge_list[j].weight < distances[edge_list[j].y]){
                distances[edge_list[j].y] = distances[edge_list[j].x] +  edge_list[j].weight;
                changed = 1;
            }
        }
        if(!changed)
            return 0;
    }
    for(j = 0; j < m; j++){
        if(distances[edge_list[j].x] +  edge_list[j].weight < distances[edge_list[j].y])
            return 1;
    }
    return 0;
}

int main(){
    int t, n, m;
    for(scanf("%d", &t); t > 0; t--){
        read_test_case(&n, &m, edge_list);
        printf("%s\n", bellman_ford(n, m, edge_list, distances) ? "possible" : "not possible");
    }
    return 0;
}
