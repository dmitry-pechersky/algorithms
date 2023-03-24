#include <stdio.h>
#include <assert.h>
#define MAX_SIZE 999
#define MAX_HEAP_SIZE 10000000

struct Path{
    int cost;
    int x;
    int y;
};

struct Heap{
    int size;
    struct Path array[MAX_HEAP_SIZE];
};

int grid[MAX_SIZE][MAX_SIZE], depth[MAX_SIZE][MAX_SIZE], dx[] = {1, 0, -1, 0}, dy[] = {0, 1, 0, -1};

struct Heap heap;

void swap(struct Path *a, struct Path *b){
    struct Path t = *a;
    *a = *b;
    *b = t;
}

void heap_push(struct Heap *heap, struct Path value){
    int i, parent_i;
    assert(heap->size < MAX_HEAP_SIZE);
    heap->array[heap->size] = value;
    i = heap->size++;
    while(i > 0){
        parent_i = (i - 1) / 2;
        if(heap->array[parent_i].cost <= heap->array[i].cost)
            break;
        swap(&heap->array[parent_i], &heap->array[i]);
        i = parent_i;
    }
}

struct Path heap_pop(struct Heap *heap){
    int i, left_i, right_i, min_i;
    struct Path value = heap->array[0];
    assert(heap->size > 0);
    heap->size--;
    heap->array[0] = heap->array[heap->size];
    i = 0;
    while(1){
        left_i = i * 2 + 1;
        right_i = i * 2 + 2;
        min_i = i;
        if(left_i < heap->size && heap->array[min_i].cost > heap->array[left_i].cost)
            min_i = left_i;
        if(right_i < heap->size && heap->array[min_i].cost > heap->array[right_i].cost)
            min_i = right_i;
        if( i == min_i)
            break;
        swap(&heap->array[i], &heap->array[min_i]);
        i = min_i;
    }
    return value;
}

void read_test_case(int *nx, int *ny, int grid[][MAX_SIZE]){
    int i, j;
    scanf("%d", nx);
    scanf("%d", ny);
    for(i = 0; i < *nx; i++)
        for(j = 0; j < *ny; j++)
            scanf("%d", &grid[i][j]);
}

int dijkstra(int nx, int ny, int grid[][MAX_SIZE], int depth[][MAX_SIZE], struct Heap *heap){
    int i, j;
    struct Path v, u;
    heap->size = 0;
    v.cost = grid[0][0]; v.x = 0; v.y = 0;
    heap_push(heap, v);
    for(i = 0; i < nx; i++)
        for(j = 0; j < ny; j++)
            depth[i][j] = -1;
    while(heap->size > 0){
        v = heap_pop(heap);
        if(depth[v.x][v.y] != -1)
            continue;
        depth[v.x][v.y] = v.cost;
        if(v.x == nx - 1 && v.y == ny - 1)
            break;
        for(i = 0; i < 4; i++){
            u.x = v.x + dx[i]; u.y = v.y + dy[i];
            if(u.x >= 0 && u.x < nx && u.y >= 0 && u.y < ny && depth[u.x][u.y] == -1){
                u.cost = v.cost + grid[u.x][u.y];
                heap_push(heap, u);
            }
        }
    }
    return depth[nx - 1][ny -1];
}

int main(){
    int t, nx, ny;
    scanf("%d", &t);
    for(; t > 0; t--){
        read_test_case(&nx, &ny, grid);
        printf("%d\n", dijkstra(nx, ny, grid, depth, &heap));
    }
    return 0;
}

