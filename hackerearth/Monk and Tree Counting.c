#include <stdio.h>
#include <stdlib.h>
#define MAX_NODES 500

int values[MAX_NODES], adj_list[MAX_NODES][3], temp[MAX_NODES];

int cmpfunc (const void * a, const void * b) {
   return ( *(int*)a - *(int*)b );
}

int lagest_smaller(int array[], int start, int end, int value){
    int mid;
    while(start < end){
        mid = (end + start) / 2 + (end + start) % 2;
        if(array[mid] >= value)
            end = mid - 1;
        else
            start = mid;
    }
    return start;
}

int count_not_smaller(int array[], int start, int end, int value){
    if(array[start] >= value)
        return end - start + 1;
    if(array[end] < value)
        return 0;
    return end - lagest_smaller(array, start, end, value);
}

int count(int values[MAX_NODES], int adj_list[MAX_NODES][3], int node, int k, int temp[MAX_NODES], int start, int *end){
    int i, triplet_cnt = 0, child_start = start;
    for(i = 1; i <= adj_list[node][0]; i++){
        triplet_cnt += count(values, adj_list, adj_list[node][i], k, temp, child_start, end);
        child_start = *end + 1;
    }
    qsort(temp + start, *end - start + 1, sizeof(int), cmpfunc);
    for(i = start; i < *end; i++){
        triplet_cnt += count_not_smaller(temp, i + 1, *end, k - values[node] - temp[i]);
    }
    *end  = *end + 1;
    temp[*end] = values[node];
    return triplet_cnt;
}

int main(){
    int i, n, k, parent, end = -1;
    scanf("%d %d", &n, &k);
    for(i = 0; i < n; i++)
        scanf("%d", &values[i]);
    for(i = 1; i < n; i++){
        scanf("%d", &parent);
        adj_list[parent - 1][++adj_list[parent - 1][0]] = i;
    }
    printf("%d\n", count(values, adj_list, 0, k, temp, 0, &end));

    return 0;
}
