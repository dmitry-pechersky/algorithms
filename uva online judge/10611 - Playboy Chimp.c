#include <stdio.h>
#include <math.h>

int heights[50000], queries[25000];

int smallest(int heights[], int i, int j, int h){
    int k;
    if(i == j)
        return i;
    k = (i + j) / 2;
    if(heights[k] > h)
        return smallest(heights, i, k, h);
    return smallest(heights, k + 1, j, h);
}

int tallest(int heights[], int i, int j, int h){
    int k;
    if(i == j)
        return i;
    k = ceil((i + j) / 2.0);
    if(heights[k] >= h)
        return tallest(heights, i, k - 1, h);
    return tallest(heights, k, j, h);
}

void read_test_case(int *n, int *q, int heights[], int queries[]){
    int i;
    scanf("%d", n);
    for(i = 0; i < *n; i++)
        scanf("%d", &heights[i]);
    scanf("%d", q);
    for(i = 0; i < *q; i++)
        scanf("%d", &queries[i]);
}

int main(){
    int n, q, i, smallest_i, tallest_i;
    read_test_case(&n, &q, heights, queries);
    for(i = 0; i < q; i++){
        smallest_i = smallest(heights, 0, n - 1, queries[i]);
        tallest_i = tallest(heights, 0, n - 1, queries[i]);
        if(heights[tallest_i] < queries[i])
            printf("%d", heights[tallest_i]);
        else
            printf("X");
        if(heights[smallest_i] > queries[i])
            printf(" %d\n", heights[smallest_i]);
        else
            printf(" X\n");
    }
    return 0;
}
