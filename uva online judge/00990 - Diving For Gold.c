#include <stdio.h>
#define MAX_TIME 1000
#define MAX_TREASURES 30

int depths[MAX_TREASURES], quantities[MAX_TREASURES], times[MAX_TREASURES], selected[MAX_TREASURES], cache[MAX_TREASURES][MAX_TIME + 1];

int read_test_case(int *n, int *t, int depths[], int quantities[], int times[]){
    int i, w;
    if(scanf("%d %d", t, &w) != 2)
        return 0;
    scanf("%d", n);
    for(i = 0; i < n; i++){
        scanf("%d %d", &depths[i], &quantities[i]);
        times[i] = 3 * w * depths[i];
    }
    return 1;
}

void knapsack(int n, int t, int depths[], int quantities[], int times[], int cache[][MAX_TIME + 1]){

}

int main(){
    int n, t;
    while(read_test_case(&n, &t, depths, quantities, times)){

    }
    return 0;
}