#include<stdio.h>
#include<stdlib.h>
#define MAX_SIZE 5000

struct Heap{
    int array[MAX_SIZE];
    int size;
};

void swap(int *array, int i, int j){
    int temp = array[i];
    array[i] = array[j];
    array[j] = temp;
}

void _heap_down(struct Heap *heap, int i){
    int smallest_i = i;
    int left_i = i * 2 + 1;
    int right_i = i * 2 + 2;
    if(left_i < heap->size && heap->array[left_i] < heap->array[smallest_i])
        smallest_i = left_i;
    if(right_i < heap->size && heap->array[right_i] < heap->array[smallest_i])
        smallest_i = right_i;
    if(smallest_i != i){
        swap(heap->array, i, smallest_i);
        _heap_down(heap, smallest_i);
    }
}

void heap_push(struct Heap *heap, int v){
    int i = heap->size++, parent_i;
    heap->array[i] = v;
    while(i != 0 &&  heap->array[parent_i = (i - 1) / 2] > heap->array[i]){
        swap(heap->array, i, parent_i);
        i = parent_i;
    }
}

int heap_pop(struct Heap *heap){
    int res = heap->array[0];
    heap->array[0] = heap->array[--heap->size];
    _heap_down(heap, 0);
    return res;
}

int read_test_case(struct Heap *heap){
    int i, n, value;
    scanf("%d", &n);
    if(n == 0)
        return 0;
    for(i = 0; i < n; i++){
        scanf("%d", &value);
        heap_push(heap, value);
    }
    return 1;
}

int add_all(struct Heap *heap){
    int a, b, cost = 0;
    while(heap->size > 1){
        a = heap_pop(heap);
        b = heap_pop(heap);
        heap_push(heap, a + b);
        cost += a + b;
    }
    return cost;
}

int main(){
    struct Heap heap;
    while(1){
        heap.size = 0;
        if(! read_test_case(&heap))
            break;
        printf("%d\n", add_all(&heap));
    }
    return 0;
}