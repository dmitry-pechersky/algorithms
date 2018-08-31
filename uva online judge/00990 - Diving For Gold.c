#include <stdio.h>
#define MAX_TIME 1000
#define MAX_TREASURES 30

int depths[MAX_TREASURES], quantities[MAX_TREASURES], times[MAX_TREASURES], selected[MAX_TREASURES], cache[MAX_TREASURES][MAX_TIME + 1];

int read_test_case(int *n, int *t, int depths[], int quantities[], int times[]){
    int i, w;
    if(scanf("%d %d", t, &w) != 2)
        return 0;
    scanf("%d", n);
    for(i = 0; i < *n; i++){
        scanf("%d %d", &depths[i], &quantities[i]);
        times[i] = 3 * w * depths[i];
    }
    return 1;
}

void knapsack(int n, int t, int depths[], int quantities[], int times[], int selected[], int cache[][MAX_TIME + 1], int *treasures){
    int i, j;
    for(j = 0; j < times[0]; j++)
        cache[0][j] = 0;
    for(j = times[0]; j <= t; j++)
        cache[0][j] = quantities[0];
    for(i = 1; i < n; i++){
        for(j = 1; j <= t; j++){
            if(times[i] <= j && cache[i - 1][j - times[i]] + quantities[i] > cache[i - 1][j]){
                cache[i][j] = cache[i - 1][j - times[i]] + quantities[i];
            }else{
                cache[i][j] = cache[i - 1][j];
            }
        }
    }
    for(i = 0; i < n; i++)
        selected[i] = 0;
    j =  t;
    *treasures = 0;
    for(i = n - 1; i > 0; i--){
        if(cache[i][j] != cache[i - 1][j]){
            j -= times[i];
            selected[i] = 1;
            *treasures += 1;
        }
    }
    if(j > 0){
        selected[0] = 1;
        *treasures += 1;
    }
}

int main(){
    int i = 0, j, n, t, treasures;
    while(read_test_case(&n, &t, depths, quantities, times)){
        knapsack(n, t, depths, quantities, times, selected, cache, &treasures);
        if(i++ > 0)
            printf("\n");
        printf("%d\n", cache[n - 1][t]);
        printf("%d\n", treasures);
        for(j = 0; j < n; j++)
            if(selected[j])
                printf("%d %d\n", depths[j], quantities[j]);
    }
    return 0;
}
