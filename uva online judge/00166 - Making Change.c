#include<stdio.h>
#include<limits.h>
#define COINS_CNT 6
#define MAX_AMOUNT 200
#define INF INT_MAX

int coins[COINS_CNT], denomination[] = {1, 2, 4, 10, 20, 40}, dp[MAX_AMOUNT];

int read_test_case(int coins[], int *transaction){
    int i, sum = 0;
    float transaction_dollars;
    for(i = 0; i < COINS_CNT; i++){
        scanf("%d", &coins[i]);
        sum += coins[i];
    }
    if(sum == 0)
        return 0;
    scanf("%f", &transaction_dollars);
    *transaction = transaction_dollars * 20;
    return 1;
}

int coin_change_greedy(int denomination[], int transaction){
    int i, cnt = 0;
    for(i = COINS_CNT - 1; i >= 0; i--){
        cnt += transaction / denomination[i];
        transaction = transaction % denomination[i];
    }
    return cnt;
}

void coin_change_dp(int coins[], int denomination[], int transaction, int dp[]){
    int i, j, k;
    for(i = 0; i <= transaction;  i++)
        dp[i] = INF;
    for(i = 0; i < COINS_CNT; i++){
        for(j = 0; j < coins[i]; j++){
            for(k = transaction; k > denomination[i]; k--){
                if(dp[k - denomination[i]] < INF && dp[k - denomination[i]] + 1 < dp[k])
                    dp[k] = dp[k - denomination[i]] + 1;
            }
            dp[denomination[i]] = 1;
        }
    }
}

int make_change(int coins[], int denomination[], int transaction, int dp[]){
    int i, min_cnt = INF, cnt;
    coin_change_dp(coins, denomination, MAX_AMOUNT, dp);
    for(i = transaction; i <= MAX_AMOUNT; i++){
        if(dp[i] < INF){
            cnt = dp[i] + coin_change_greedy(denomination, i - transaction);
            if(cnt < min_cnt)
                min_cnt = cnt;
        }
    }
    return min_cnt;
}

int main(){
    int transaction;
    while(read_test_case(coins, &transaction)){
        printf("%*d\n", 3, make_change(coins, denomination, transaction, dp));
    }
    return 0;
}
