#include<stdio.h>
#include<limits.h>
#define MAX_NODES 200
#define MAX_EDGES 40000
#define INF INT_MAX
#define CUBE(x) ((x)*(x)*(x))

struct Edge{
    int x;
    int y;
    int weight;
};

struct Edge edge_list[MAX_EDGES];

int distance[MAX_NODES + 1], is_negative_cycle[MAX_NODES + 1], query[MAX_NODES], busyness[MAX_NODES + 1];

int read_test_case(int *n, int *m, struct Edge edge_list[], int *q, int query[], int busyness[]){
    int i;
    if(scanf("%d", n) != 1)
        return 0;
    for(i = 1; i <= *n; i++)
        scanf("%d", &busyness[i]);
    scanf("%d", m);
    for(i = 0; i < *m; i++){
        scanf("%d %d", &edge_list[i].x, &edge_list[i].y);
        edge_list[i].weight = CUBE(busyness[edge_list[i].y] - busyness[edge_list[i].x]);
    }
    scanf("%d", q);
    for(i = 0; i < *q; i++)
        scanf("%d", &query[i]);
    return 1;
}

void bellman_ford(int n, int m, struct Edge edge_list[], int distance[], int is_negative_cycle[]){
    int i, j, changed;
    for(i = 1; i <= n; i++){
        distance[i] = INF;
        is_negative_cycle[i] = 0;
    }
    distance[1] = 0;
    for(i = 0; i < 2 * n - 1; i++){
        changed = 0;
        for(j = 0; j < m; j++){
            if(distance[edge_list[j].x] < INF && (distance[edge_list[j].x] + edge_list[j].weight) < distance[edge_list[j].y]){
                distance[edge_list[j].y] = distance[edge_list[j].x] + edge_list[j].weight;
                changed = 1;
                if(i >= n - 1)
                    is_negative_cycle[edge_list[j].y] = 1;
            }
        }
        if(!changed)
            break;
    }
}

int main(){
    int n, m, q, i = 1, j;
    while(read_test_case(&n, &m, edge_list, &q, query, busyness)){
        printf("Set #%d\n", i);
        if(n > 0 && q > 0){
            bellman_ford(n, m, edge_list, distance, is_negative_cycle);
            for(j = 0; j < q; j++)
                if(query[j] <= 0 || query[j] > n || distance[query[j]] == INF || distance[query[j]] < 3 || is_negative_cycle[query[j]])
                    printf("?\n");
                else
                    printf("%d\n", distance[query[j]]);
        }
        i++;
    }
    return 1;
}