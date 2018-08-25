#include <stdio.h>
#define MAX_STOPS 20000

int roads[MAX_STOPS - 1];

int read_test_case(int *s, int roads[]){
    int i;
    scanf("%d", s);
    for(i = 0; i < *s - 1; i++)
        scanf("%d", &roads[i]);
}

void max_contiguous_subarray(int n, int array[], int *max_sum, int *max_i, int *max_j){
    int i = 0, j;
    for(j = 1; j < n; j++)
        if(array[j - 1] > 0)
            array[j] += array[j - 1];
    *max_sum = array[0]; *max_i = 0; *max_j = 0;
    for(j = 1; j < n; j++){
        if(array[j - 1] < 0)
            i = j;
        if(array[j] > *max_sum || (array[j] == *max_sum && j - i > *max_j - *max_i)){
            *max_sum = array[j];
            *max_i = i;
            *max_j = j;
        }
    }
}

int main(){
    int t, i, s, max_sum, max_i, max_j;
    scanf("%d", &t);
    for(i = 1; i <= t; i++){
        read_test_case(&s, roads);
        max_contiguous_subarray(s - 1, roads, &max_sum, &max_i, &max_j);
        if(max_sum > 0)
            printf("The nicest part of route %d is between stops %d and %d\n", i, max_i + 1, max_j + 2);
        else
            printf("Route %d has no nice parts\n", i);
    }
    return 0;
}
