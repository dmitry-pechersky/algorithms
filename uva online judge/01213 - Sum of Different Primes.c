#include<stdio.h>
#define MAX_K 14
#define MAX_N 1120

int numbers[MAX_N + 1], dp[MAX_N + 1][MAX_K + 1];

int prime_numbers(int numbers[]){
    int i, j, numbers_n = 0;
    for(i = 2; i <= MAX_N; i++)
        numbers[i] = 1;
    for(i = 2; i < MAX_N; i++){
        if(numbers[i]){
            numbers[numbers_n++] = i;
            for(j = 2 * i; j <= MAX_N; j += i)
                numbers[j] = 0;
        }
    }
    return numbers_n;
}

void knapsack(int numbers_n, int numbers[], int dp[][MAX_K + 1]){
    int i, j, k;
    for(i = 0; i < numbers_n; i++){
        for(j = MAX_N; j >= 2; j--){
            if(numbers[i] == j)
                dp[j][1] = 1;
            else if(numbers[i] < j)
                for(k = 1; k <= MAX_K; k++)
                    dp[j][k] += dp[j - numbers[i]][k - 1];
        }
    }
}

int main(){
    int n, k;
    knapsack(prime_numbers(numbers), numbers, dp);
    while(scanf("%d %d", &n, &k) && n && k)
        printf("%d\n", dp[n][k]);
    return 0;
}
