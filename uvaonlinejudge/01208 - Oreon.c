#include<stdio.h>
#include<stdlib.h>

struct Edge{
    int v;
    int u;
    int w;
};

int n, edge_list_n, *parents = NULL, *sizes = NULL;

struct Edge *edge_list = NULL, *mst_edge_list = NULL;

int edge_comp(const void *a, const void *b){
    struct Edge *edge_a = (struct Edge *) a;
    struct Edge *edge_b = (struct Edge *) b;
    if(edge_a->w == edge_b->w)
        return 0;
    return edge_a->w <  edge_b->w ? -1 : 1;
}

void disjointset_init(int n){
    int i;
    parents = realloc(parents, sizeof(int) * n);
    sizes = realloc(sizes, sizeof(int) * n);
    for(i = 0; i < n; i++){
        parents[i] = i;
        sizes[i] = 1;
    }
}

int disjointset_find(int x){
    if(parents[x] != x){
        parents[x] = disjointset_find(parents[x]);
    }
    return parents[x];
}

void disjointset_union(int x, int y){
    int root_x = disjointset_find(x);
    int root_y = disjointset_find(y);
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

void kruskal(){
    int i, j = 0;
    disjointset_init(n);
    mst_edge_list = realloc(mst_edge_list, sizeof(struct Edge) * (n - 1));
    qsort(edge_list, edge_list_n, sizeof(struct Edge), edge_comp);
    for(i = 0; i < edge_list_n; i++){
        if(disjointset_find(edge_list[i].v) != disjointset_find(edge_list[i].u)){
            disjointset_union(edge_list[i].v, edge_list[i].u);
            mst_edge_list[j++] = edge_list[i];
            if(j >= n - 1)
                break;
        }
    }
}

void read_test_case(){
    struct Edge edge;
    scanf("%d", &n);
    edge_list = realloc(edge_list, sizeof(struct Edge) * n * n);
    edge_list_n = 0;
    for(edge.v = 0; edge.v < n; edge.v++){
        for(edge.u = 0; edge.u < n; edge.u++){
            scanf("%d,", &edge.w);
            if(edge.u > edge.v && edge.w > 0){
                edge_list[edge_list_n++] = edge;
            }
        }
    }
}

int main(){
    int i, j, t;
    scanf("%d", &t);
    for(i = 0; i < t; i++){
        read_test_case();
        kruskal();
        printf("Case %d:\n", i + 1);
        for(j = 0; j < n - 1; j++)
            printf("%c-%c %d\n", (char) (mst_edge_list[j].v + 65), (char) (mst_edge_list[j].u + 65), mst_edge_list[j].w);
    }
    return 0;
}