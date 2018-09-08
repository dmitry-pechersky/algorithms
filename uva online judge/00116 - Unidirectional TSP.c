#include <stdio.h>
#include <limits.h>
#define MAX_N 10
#define MAX_M 100
#define INF INT_MAX

int matrix[MAX_N][MAX_M], dp[MAX_N][MAX_M], next[MAX_N][MAX_M];

int read_test_case(int *n, int *m, int matrix[][MAX_M]){
    int i, j;
    if(scanf("%d %d", n, m) != 2)
        return 0;
    for(i = 0; i < *n; i++)
        for(j = 0; j < *m; j++)
            scanf("%d", &matrix[i][j]);
    return 1;
}

void minimal_path_dp(int n, int m, int matrix[][MAX_M], int dp[][MAX_M], int next[][MAX_M]){
    int i, j, k, next_i;
    for(i = 0; i < n; i++)
        dp[i][m - 1] = matrix[i][m - 1];
    for(j = m - 2; j >= 0; j--){
        for(i = 0; i < n; i++){
            dp[i][j] = INF;
            next[i][j] = INF;
            for(k = i - 1; k <= i + 1; k++){
                next_i = (k + n) % n;
                if(dp[next_i][j + 1] < dp[i][j] || (dp[next_i][j + 1] == dp[i][j] &&  next_i < next[i][j])){
                    dp[i][j] = dp[next_i][j + 1];
                    next[i][j] = next_i;
                }
            }
            dp[i][j] += matrix[i][j];
        }
    }
}

int main(){
    int n, m, i, j, min_i;
    while(read_test_case(&n, &m, matrix)){
        minimal_path_dp(n, m, matrix, dp, next);
        min_i = 0;
        for(j = 0; j < n; j++)
            if(dp[j][0] < dp[min_i][0])
                min_i = j;
        i = min_i;
        for(j = 0; j < m - 1; j++){
            printf("%d ", i + 1);
            i = next[i][j];
        }
        printf("%d\n", i + 1);
        printf("%d\n", dp[min_i][0]);
    }
    return 0;
}
