#include<stdio.h>
#define N 100000

struct AdjacencyList{
    int n, m;
    int vertex[N];
    int edge[N][2];
};

struct Stack{
    int n;
    int array[N];
};

struct AdjacencyList adj_list;

struct Stack stack;

int num[N], low[N], on_stack[N], in_edges[N], components[N];

void read_test_case(struct AdjacencyList *adj_list){
    int i, v, u;
    scanf("%d %d", &adj_list->n, &adj_list->m);
    for(v = 0; v < adj_list->n; v++)
        adj_list->vertex[v] = -1;
    for(i = 0; i < adj_list->m; i++){
        scanf("%d %d", &v, &u);
        adj_list->edge[i][0] = u - 1;
        adj_list->edge[i][1] = adj_list->vertex[v - 1];
        adj_list->vertex[v - 1] = i;
    }
}

void tarjan(struct AdjacencyList *adj_list, int v, int *depth, int num[], int low[], struct Stack *stack, int on_stack[]){
    int i, u;
    num[v] = low[v] = *depth = *depth + 1;
    stack->array[(stack->n)++] = v;
    on_stack[v] = 1;
    for(i = adj_list->vertex[v]; i != -1; i = adj_list->edge[i][1]){
        u = adj_list->edge[i][0];
        if(num[u] == -1)
            tarjan(adj_list, u, depth, num, low, stack, on_stack);
        if(on_stack[u] && low[v] > low[u])
            low[v] = low[u];
    }
    if(num[v] == low[v]){
        while(1){
            u = stack->array[--(stack->n)];
            on_stack[u] = 0;
            if(v == u)
                break;
            low[u] = num[v];
        }
    }
}

int scc_tarjan(struct AdjacencyList *adj_list, int num[], int low[], struct Stack *stack, int on_stack[], int in_edges[N], int components[N]){
    int i, v, u, depth = -1, cnt = 0;
    stack->n = 0;
    for(v = 0; v < adj_list->n; v++){
        num[v] = -1;
        components[v] = in_edges[v] = on_stack[v] = 0;
    }
    for(v = 0; v < adj_list->n; v++)
        if(num[v] == -1)
            tarjan(adj_list, v, &depth, num, low, stack, on_stack);
    for(v = 0; v < adj_list->n; v++){
        if(!components[low[v]]){
            components[low[v]] = 1;
            cnt++;
        }
        for(i = adj_list->vertex[v]; i != -1; i = adj_list->edge[i][1]){
            u = adj_list->edge[i][0];
            if(low[v] != low[u] && !in_edges[low[u]]){
                in_edges[low[u]] = 1;
                cnt--;
            }
        }
    }
    return cnt;
}

int main(){
    int i;
    for(scanf("%d", &i); i > 0; i--){
        read_test_case(&adj_list);
        printf("%d\n", scc_tarjan(&adj_list, num, low, &stack, on_stack, in_edges, components));
    }
}
