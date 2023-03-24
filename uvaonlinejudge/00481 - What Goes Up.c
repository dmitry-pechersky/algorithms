#include<stdio.h>
#define MAX_SIZE 100000

int array[MAX_SIZE], dp[MAX_SIZE], parent[MAX_SIZE];

int read_test_case(int array[]){
    int i = 0;
    while(scanf("%d", &array[i]) == 1)
        i++;
    return i;
}

int smallest_larger(int array[], int dp_n, int dp[], int value){
    int a = 0, b = dp_n - 1, c;
    while(a != b){
        c = (a + b) / 2;
        if(array[dp[c]] >= value)
            b = c;
        else
            a = c + 1;
    }
    return b;
}

int longest_increasing_subsequence(int n, int array[], int dp[], int parent[]){
    int i, j, dp_n = 1;
    dp[0] = 0;
    for(i = 1; i < n; i++){
        if(array[i] > array[dp[dp_n - 1]]){
            parent[i] = dp[dp_n - 1];
            dp[dp_n] = i;
            dp_n++;
        }else if(array[i] < array[dp[dp_n - 1]]){
            j = smallest_larger(array, dp_n, dp, array[i]);
            if(array[i] < array[dp[j]]){
                dp[j] = i;
                if(j > 0)
                    parent[i] = dp[j - 1];
            }
        }
    }
    for(i = dp_n - 2; i >= 0; i--)
        dp[i] = parent[dp[i + 1]];
    return dp_n;
}

int main(){
    int i, n, dp_n;
    n = read_test_case(array);
    dp_n = longest_increasing_subsequence(n, array, dp, parent);
    printf("%d\n-\n", dp_n);
    for(i = 0; i < dp_n; i++)
        printf("%d\n", array[dp[i]]);
    return 0;
}
