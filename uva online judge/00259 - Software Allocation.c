#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <assert.h>
#define N 38
#define SOURCE 0
#define SINK 37
#define INF INT_MAX

struct Queue{
    int size;
    int idx;
    int array[N];
};

int adj_matrix[N][N], parent[N];

struct Queue queue;

void queue_push(struct Queue *queue, int value){
    assert(queue->size < N);
    queue->array[(queue->idx + queue->size) % N] = value;
    queue->size ++;
}

int queue_pop(struct Queue *queue){
    assert(queue->size > 0);
    int value = queue->array[queue->idx];
    queue->idx = (queue->idx + 1) % N;
    queue->size --;    
    return value;
}

int read_test_case(int adj_matrix[][N]){
    int v, u, end_of_tests = 1;
    for(v = 0; v < N; v++)
        for(u = 0; u < N; u++)
            adj_matrix[v][u] = 0;
    while(1){
        v = getchar();
        if(v == '\n' || v == EOF){
            break;
        }
        end_of_tests = 0;
        v =  v - 65 + 1;
        adj_matrix[SOURCE][v] = getchar() - 48;
        getchar();
        while((u = getchar()) != ';'){
            u = u - 48 + 27;
            adj_matrix[u][SINK] = 1;
            adj_matrix[v][u] = INF;
        }
        getchar();
    }
    return !end_of_tests;
}

void bfs(int adj_matrix[][N], int n, int start, int end, int parent[], struct Queue *queue){
    int v, u;
    for(v = 0; v < N; v++)
        parent[v] = -1;
    parent[start] = start;
    queue->size = queue->idx = 0;
    queue_push(queue, start);
    while(queue->size > 0){
        v = queue_pop(queue);
        for(u = 0; u < N; u++){
            if(adj_matrix[v][u] > 0 && parent[u] == -1){
                queue_push(queue, u);
                parent[u] = v;
                if(u == end)
                    return;
            }
        }
    }
}

void edmonds_karp(int adj_matrix[][N], int n, int source, int sink, int parent[], struct Queue *queue){
    int v, flow, full_flow = 0;
    while(1){
        bfs(adj_matrix, n, source, sink, parent, queue);
        if(parent[sink] == -1)
            break;
        v = sink;
        flow = INF;
        while(v != source){
            if(adj_matrix[parent[v]][v] < flow)
                flow = adj_matrix[parent[v]][v];
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
    return;
}

int main(){
    int v, u, success;
    while(read_test_case(adj_matrix)){
        edmonds_karp(adj_matrix, N, SOURCE, SINK, parent, &queue);
        success = 1;
        for(v = 0; v < N; v++){
            if(adj_matrix[0][v] != 0){
                success = 0;
                break;
            }
        }
        if(success){
            for(v = 27 ; v < 37; v++){
                success = 0;
                for(u = 1; u < 27; u++){
                    if(adj_matrix[v][u] > 0){
                        printf("%c", (u + 65 - 1));
                        success = 1;
                        break;
                    }
                }                
                if(!success)
                    printf("_");
            }
            printf("\n");
            
        }else{
            printf("!\n");
        }
    }
    return 0;
}