#include <stdio.h>
#include <assert.h>
#include <limits.h>
#define MAX_NODES 100
#define INF INT_MAX;

struct Queue{
    int idx;
    int size;
    int array[MAX_NODES];
};

int adj_matrix[MAX_NODES][MAX_NODES], parent[MAX_NODES];

struct Queue queue;

void queue_push(struct Queue *queue, int value){
    assert(queue->size < MAX_NODES);
    queue->array[(queue->idx + queue->size) % MAX_NODES] = value;
    queue->size++;
}

int queue_pop(struct Queue *queue){
    assert(queue->size > 0);
    int value = queue->array[queue->idx];
    queue->idx = (queue->idx + 1) % MAX_NODES;
    queue->size --;
    return value;
}

int read_test_case(int *n, int adj_matrix[][MAX_NODES], int *source, int *sink){
    int m, v, u, weight;
    scanf("%d", n);
    if(*n == 0)
        return 0;
    for(v = 0; v < *n; v++)
        for(u = 0; u < *n; u++)
            adj_matrix[v][u] = 0;
    scanf("%d %d %d", source, sink, &m);
    (*source)--; (*sink)--;
    for(; m > 0; m--){
        scanf("%d %d %d", &v, &u, &weight);
        adj_matrix[v - 1][u - 1] += weight;
        adj_matrix[u - 1][v - 1] += weight;
    }
    return 1;
}

void bfs(int n, int adj_matrix[][MAX_NODES], int start, int end, int parent[], struct Queue *queue){
    int v, u;
    for(v = 0; v < n; v++)
        parent[v] = -1;
    parent[start] = start;
    queue->idx = queue->size = 0;
    queue_push(queue, start);
    while(queue->size > 0){
        v = queue_pop(queue);
        for(u = 0; u < n; u++){
            if(adj_matrix[v][u] > 0 && parent[u] == -1){
                parent[u] = v;
                if(u == end)
                    return;
                queue_push(queue, u);
            }
        }
    }
}

int edmonds_karp(int n, int adj_matrix[][MAX_NODES], int source, int sink, int parent[], struct Queue *queue){
    int full_flow = 0, flow, v;
    while(1){
        bfs(n, adj_matrix, source, sink, parent, queue);
        if(parent[sink] == -1)
            break;
        flow = INF;
        v = sink;
        while(v != source){
            flow = adj_matrix[parent[v]][v] < flow ? adj_matrix[parent[v]][v] : flow;
            v = parent[v];
        }
        v = sink;
        while(v != source){
            adj_matrix[parent[v]][v] -= flow;
            adj_matrix[v][parent[v]] += flow;
            v = parent[v];
        }
        full_flow += flow;
    }
    return full_flow;
}

int main(){
    int i = 1, n, source, sink;
    while(read_test_case(&n, adj_matrix, &source, &sink)){
        printf("Network %d\n", i);
        printf("The bandwidth is %d.\n\n", edmonds_karp(n, adj_matrix, source, sink, parent, &queue));
        i++;
    }
    return 0;
}