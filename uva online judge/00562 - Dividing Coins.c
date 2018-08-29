#include <stdio.h>
#define MAX_AMOUNT_VALUES 25001
#define MAX_COINS 100
#define MAX(x, y) (((x) > (y)) ? (x) : (y))

int coins[MAX_COINS], cache[MAX_AMOUNT_VALUES];

void read_test_case(int *n, int coins[]){
    int i;
    scanf("%d", n);
    for(i = 0; i < *n; i++)
        scanf("%d", &coins[i]);
}

int sum(int n, int array[]){
    int i, sum = 0;
    for(i = 0; i < n; i++)
        sum += array[i];
    return sum;
}

int knapsack(int n, int array[], int value, int cache[]){
    int i, j;
    for(i = 0; i < array[0]; i++)
        cache[i] = 0;
    for(i = array[0]; i <= value; i++)
        cache[i] = array[0];
    for(i = 1; i < n; i++){
        for(j = value; j >= 0; j--){
            if(array[i] < j){
                cache[j] = MAX(cache[j], cache[j - array[i]] + array[i]);
            }else if(array[i] == j){
                cache[j] = array[i];
            }
        }
    }
    return cache[value];
}

int main(){
    int t, n, total_sum, half_floor;
    scanf("%d", &t);
    while(t-- > 0){
        read_test_case(&n, coins);
        total_sum = sum(n, coins);
        half_floor = total_sum / 2;
        printf("%d\n", total_sum - 2 * knapsack(n, coins, half_floor, cache));
    }
    return 0;
}
