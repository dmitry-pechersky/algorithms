#include <stdio.h>
#include <assert.h>
#include <string.h>
#define MAX_NODES 2500
#define MAX_EDGES 15
#define MAX_SOURCES 60

struct Queue{
    int array[MAX_NODES];
    int head;
    int size;
};

int adj_list[MAX_NODES][MAX_EDGES + 1], depths[MAX_NODES], cnts[MAX_NODES];

struct Queue queue;

void queue_push(struct Queue *queue, int v){
    assert(queue->size < MAX_NODES);
    queue->array[(queue->head + queue->size) % MAX_NODES] = v;
    queue->size ++;
}

int queue_pop(struct Queue *queue){
    assert(queue->size > 0);
    int res = queue->array[queue->head];
    queue->size --;
    queue->head = (queue->head + 1) % MAX_NODES;
    return res;
}

void read_test_case(int adj_list[MAX_NODES][MAX_EDGES + 1], int *n, int sources[MAX_SOURCES], int *sources_n){
    int i, j;
    scanf("%d", n);
    for(i = 0; i < *n; i++){
        scanf("%d", &adj_list[i][0]);
        for(j = 1; j <= adj_list[i][0]; j++)
            scanf("%d", &adj_list[i][j]);
    }
    scanf("%d", sources_n);
    for(i = 0; i < *sources_n; i++){
        scanf("%d", &sources[i]);
    }
}

int bfs(int adj_list[MAX_NODES][MAX_EDGES + 1], int n, int v, struct Queue *queue, int depths[], int cnts[]){
    int i, u, max_cnt_depth = 0;
    for(i = 0; i < n; i++)
        depths[i] = -1;
    memset(cnts, 0, MAX_NODES * sizeof(int));
    queue->head = queue->size = 0;
    depths[v] = 0; queue_push(queue, v);
    while(queue->size > 0){
        v = queue_pop(queue);
        for(i = 1; i <= adj_list[v][0]; i++){
            u = adj_list[v][i];
            if(depths[u] == -1){
                depths[u] = depths[v] + 1;
                queue_push(queue, u);
                cnts[depths[u]] ++;
                if(cnts[depths[u]] > cnts[max_cnt_depth])
                    max_cnt_depth = depths[u];
            }
        }
    }
    return max_cnt_depth;
}

int main(){
    int i, n, sources_n, sources[MAX_SOURCES], max_cnt_depth;
    read_test_case(adj_list, &n, sources, &sources_n);
    for(i = 0; i < sources_n; i++){
        max_cnt_depth = bfs(adj_list, n, sources[i], &queue, depths, cnts);
        if(cnts[max_cnt_depth] > 0)
            printf("%d %d\n",  cnts[max_cnt_depth], max_cnt_depth);
        else
            printf("0\n");
    }
    return 0;
}