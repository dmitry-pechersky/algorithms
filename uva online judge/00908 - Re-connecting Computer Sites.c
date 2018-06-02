#include <stdio.h>
#include <stdlib.h>

struct Edge{
    int v;
    int u;
    int w;
};

int n, edge_list_n, old_cost, new_cost, *parents = NULL, *sizes = NULL;

struct Edge *edge_list = NULL;

int read_test_case(){
    int v, u, w, i, k, m;
    if(scanf("%d", &n) == EOF)
        return 0;
    old_cost = 0;
    for(i = 0; i < n -1; i++){
        scanf("%d %d %d", &v, &u, &w);
        old_cost += w;
    }
    scanf("%d", &k);
    edge_list = (struct Edge *) realloc(edge_list, sizeof(struct Edge) * k);
    for(i = 0; i < k; i++){
        scanf("%d %d %d", &edge_list[i].v, &edge_list[i].u, &edge_list[i].w);
    }
    scanf("%d", &m);
    edge_list_n = k + m;
    edge_list = (struct Edge *) realloc(edge_list, sizeof(struct Edge) * (edge_list_n));
    for(i = k; i < edge_list_n; i++){
        scanf("%d %d %d", &edge_list[i].v, &edge_list[i].u, &edge_list[i].w);
    }
    return 1;
}

void disjointset_init(int n){
    int i;
    for(i = 0; i < n; i++){
        parents[i] = i;
        sizes[i] = 1;
    }
}

int disjointset_find(int x){
    if(parents[x] != x)
        parents[x] = disjointset_find(parents[x]);
    return parents[x];
}

void disjointset_union(int x, int y){
    int root_x, root_y;
    root_x = disjointset_find(x); 
    root_y = disjointset_find(y);
    if(root_x != root_y){
        if(sizes[root_x] > sizes[root_y]){
            parents[root_y] = root_x;
            sizes[root_x] += sizes[root_y];
        }else{
            parents[root_x] = root_y;
            sizes[root_y] += sizes[root_x];
        }
    }
}

int compare_edge(const void* a, const void* b){
    if( ((struct Edge *) a)->w == ((struct Edge *) b)->w)
        return 0;
    return ((struct Edge *) a)->w < ((struct Edge *) b)->w ? -1 : 1;
}

void kruskal(){
    int i;
    qsort(edge_list, edge_list_n, sizeof(struct Edge), compare_edge);
    parents = realloc(parents, sizeof(int) * (n + 1));
    sizes = realloc(sizes, sizeof(int) * (n + 1));
    disjointset_init( n + 1);
    new_cost = 0;
    for(i = 0; i < edge_list_n; i++){
        if(disjointset_find(edge_list[i].v) != disjointset_find(edge_list[i].u)){
            disjointset_union(edge_list[i].v, edge_list[i].u);
            new_cost += edge_list[i].w;
            if(sizes[disjointset_find(edge_list[i].v)] == n)
                break;
        }
    }
}

int main(){
    int i = 0;
    while(read_test_case()){
        kruskal();
        if(i++ > 0)
            printf("\n");
        printf("%d\n%d\n", old_cost, new_cost);
    }
    return 0;
}