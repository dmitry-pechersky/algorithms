#include <stdio.h>
#include <math.h>
#define MAX_N 200

struct Edge{
    int weight;
    int v; 
    int u;
};

int stones[MAX_N][2];

struct Edge edges[MAX_N * MAX_N];

int read_test_case(int stones[][2]){
    int i, n;
    scanf("%d", &n);
    for(i = 0; i < n; i++)
        scanf("%d %d", &stones[i][0], &stones[i][1]);
    return n;
}

int edge_comp(const void * a, const void * b){
    return  ((struct Edge *) a)->weight - ((struct Edge *) b)->weight;
}

int disjoint_set_find(int parents[], int x){
    if(parents[x] != parents[parents[x]])
        parents[x] = disjoint_set_find(parents, parents[x]);
    return parents[x];
}

void disjoint_set_union(int parents[], int x, int y){
    int root_x = disjoint_set_find(parents, x) , root_y = disjoint_set_find(parents, y);
    if(root_x != root_y)
        parents[root_x] = root_y;
}

int kruskal(int n, struct Edge edges[], int edges_n){
    int i, v, parents[MAX_N];
    qsort(edges, edges_n, sizeof(struct Edge), edge_comp);
    for(v = 0; v < n; v++)
        parents[v] = v;
    for(i = 0; i < edges_n; i++){
        disjoint_set_union(parents, edges[i].v, edges[i].u);
        if(disjoint_set_find(parents, 0) == disjoint_set_find(parents, 1))
            return edges[i].weight;
    }
    return -1;
}

int main(){
    int i = 0, n, v, u, edges_n;
    while((n = read_test_case(stones)) > 0){
        edges_n = 0;
        for(v = 0; v < n - 1; v++){
            for(u = v + 1; u < n; u++){
                edges[edges_n].weight = (stones[v][0] - stones[u][0]) * (stones[v][0] - stones[u][0]) + (stones[v][1] - stones[u][1]) * (stones[v][1] - stones[u][1]);
                edges[edges_n].v = v; edges[edges_n].u = u;
                edges_n++;
            }
        }
        printf("Scenario #%d\n", ++i);
        printf("Frog Distance = %.3f\n\n", sqrt(kruskal(n, edges, edges_n)));
    }
    return 0;
}
