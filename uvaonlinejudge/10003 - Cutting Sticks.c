#include<stdio.h>
#include<limits.h>
#define MAX_SIZE 51
#define INF INT_MAX

int cuts[MAX_SIZE], dp[MAX_SIZE][MAX_SIZE];

int read_test_case(int cuts[]){
    int l, n, i;
    scanf("%d", &l);
    if(l == 0)
        return 0;
    scanf("%d", &n);
    for(i = 1; i <= n; i++)
        scanf("%d", &cuts[i]);
    cuts[n + 1] = l;
    return n + 2;
}

int cutting_sticks_dp(int cuts[], int i, int j, int dp[][MAX_SIZE]){
    int k, value, min_value = INF;
    if(j - i == 1)
        return 0;
    if(dp[i][j] == 0){
        for(k = i + 1; k < j; k++){
            value = cutting_sticks_dp(cuts, i, k, dp) + cutting_sticks_dp(cuts, k, j, dp);
            if(value < min_value)
                min_value = value;
        }
        dp[i][j] = min_value + cuts[j] - cuts[i];
    }
    return dp[i][j];
}

int main(){
    int i, j, n;
    while((n = read_test_case(cuts)) > 0){
        for(i = 0; i < n; i++)
            for(j = 0; j < n; j++)
                dp[i][j] = 0;
        printf("The minimum cutting is %d.\n", cutting_sticks_dp(cuts, 0, n - 1, dp));
    }
    return 0;
}
