#include<stdio.h>
#include<stdlib.h>

int n, pqueue_size;
int *nums = NULL, *pqueue = NULL;

void swap(int *array, int i, int j){
    int temp = array[i];
    array[i] = array[j];
    array[j] = temp;
}

void pqueue_init(int max_size){
    pqueue = realloc(pqueue, sizeof(int) * max_size);
    pqueue_size = 0;
}

void _pqueue_up(){
    int i = pqueue_size - 1, parent_i;
    while(i != 0 &&  pqueue[parent_i = (i - 1) / 2] > pqueue[i]){
        swap(pqueue, i, parent_i);
        i = parent_i;
    }
}

void _pqueue_down(){
    int i = 0, left_i, right_i;
    while( i < pqueue_size){
        left_i = i * 2 + 1;
        right_i = i * 2 + 2;
        if(right_i < pqueue_size && pqueue[right_i] < pqueue[i] && pqueue[right_i] < pqueue[left_i]){
            swap(pqueue, i, right_i);
            i = right_i;
        }else if(left_i < pqueue_size && pqueue[left_i] < pqueue[i]){
            swap(pqueue, i, left_i);
            i = left_i;            
        }else{
            break;
        }
    }
}

void pqueue_push(int v){
    pqueue[pqueue_size++] = v;
    _pqueue_up();
}

int pqueue_pop(){
    int res = pqueue[0];
    pqueue[0] = pqueue[--pqueue_size];
    _pqueue_down();
    return res;
}

int read_test_case(){
    int i;
    scanf("%d", &n);
    if(n == 0)
        return 0;
    nums = realloc(nums, sizeof(int) * n);
    for(i = 0; i < n; i++){
        scanf("%d", &nums[i]);
    }
    return 1;
}

int add_all(){
    int i, a, b, cost = 0;
    pqueue_init(n);
    for(i = 0; i < n; i++){
        pqueue_push(nums[i]);
    }
    while(pqueue_size > 1){
        a = pqueue_pop();
        b = pqueue_pop();
        pqueue_push(a + b);
        cost += a + b;
    }
    return cost;
}

int main(){
    while(read_test_case()){
        printf("%d\n", add_all());
    }
    return 0;
}