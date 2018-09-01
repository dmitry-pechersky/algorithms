#include <stdio.h>
#define MAX_VALUE 6000

void coin_change(int n, int coins[], int amount, long long cache[]){
    int i, j;
    for(j = coins[0]; j <= amount; j += coins[0])
        cache[j] = 1;
    for(i = 1; i < n; i++)
        for(j = 1; j <= amount; j++)
            if(coins[i] < j)
                cache[j] += cache[j - coins[i]];
            else if(coins[i] == j)
                cache[j] += 1;
}

int main(){
    float value;
    int n = 11, coins[] = {2000, 1000, 400, 200, 100, 40, 20, 10, 4, 2, 1};
    long long cache[MAX_VALUE + 1] = {0};
    coin_change(n, coins, MAX_VALUE, cache);
    while(scanf("%f", &value) && value != 0){
        printf("%6.2f %16lld\n", value, cache[(int) (value * 100 / 5 + 0.1)]);
    }
    return 0;
}