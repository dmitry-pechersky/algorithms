#include <stdio.h>
#include <assert.h>
#define MAX_SIZE 100

struct Stack{
    int size;
    int array[MAX_SIZE];
};

struct Queue{
    int size;
    int start;
    int array[MAX_SIZE];

};

void stack_push(struct Stack *stack, int value){
    assert(stack->size < MAX_SIZE);
    stack->array[stack->size ++] = value;
}

int stack_pop(struct Stack *stack){
    assert(stack->size > 0);
    stack->size --;
    return stack->array[stack->size];
}

int stack_top(struct Stack *stack){
    assert(stack->size > 0);
    return stack->array[stack->size - 1];
}

void queue_push(struct Queue *queue, int value){
    assert(queue->size < MAX_SIZE);
    queue->array[(queue->start + queue->size) % MAX_SIZE] = value;
    queue->size ++;
}

int queue_pop(struct Queue *queue){
    assert(queue->size > 0);
    int value = queue->array[queue->start];
    queue->start = (queue->start + 1) % MAX_SIZE;
    queue->size --;
    return value;
}

void read_test_case(int *n, int *carrier_limit, int *station_limit, struct Queue stations[]){
    int i, j, value;
    scanf("%d %d %d", n, carrier_limit, station_limit);
    for(i = 0; i < *n; i++){
        for(scanf("%d", &j); j > 0; j--){
            scanf("%d", &value);
            queue_push(&stations[i], value - 1);
        }
    }
}

int main(){
    int i, t, n, carrier_limit, station_limit, unfinished_cargos, minutes;
    struct Queue stations[MAX_SIZE];
    struct Stack carrier;
    for(scanf("%d", &t); t > 0; t--){
        carrier.size = 0;
        for(i = 0; i < MAX_SIZE; i++)
            stations[i].size = stations[i].start = 0;
        read_test_case(&n, &carrier_limit, &station_limit, stations);
        minutes = unfinished_cargos = 0;
        for(i = 0; i < n; i++)
            unfinished_cargos += stations[i].size;
        i = 0;
        while(unfinished_cargos > 0){
            while(carrier.size > 0 && (stations[i].size < station_limit || stack_top(&carrier) == i)){
                if(stack_top(&carrier) == i){
                    unfinished_cargos --;
                }else{
                    queue_push(&stations[i], stack_top(&carrier));
                }
                stack_pop(&carrier);
                minutes ++;
            }
            while(carrier.size < carrier_limit && stations[i].size > 0){
                stack_push(&carrier, queue_pop(&stations[i]));
                minutes ++;
            }
            minutes += 2;
            i = (i + 1) % n;
        }
        printf("%d\n", minutes - 2);
    }
    return 0;
}
