#include<stdio.h>
#define MAX_N 1000000

int sequence[MAX_N], dp[MAX_N];

int read_test_case(int sequence[]){
    int i, n;
    scanf("%d", &n);
    for(i = 0; i < n; i++)
        scanf("%d\n", &sequence[i]);
    return n;
}

int smallest_greater_dc(int array[], int a, int b, int value){
    int c;
    while(a != b){
        c = (a + b) / 2;
        if(array[c] < value)
            a = c + 1;
        else 
            b = c;
    }
    return b;
}

int longest_increasing_subsequence(int n, int sequence[], int dp[]){
    int i, dp_n = 1;
    dp[0] = sequence[0];
    for(i = 1; i < n; i++){
        if(sequence[i] > dp[dp_n - 1]){
            dp[dp_n++] = sequence[i];
        }else if(sequence[i] < dp[dp_n - 1]){
            dp[smallest_greater_dc(dp, 0, dp_n - 1, sequence[i])] = sequence[i];
        }
    }
    return dp_n;
}

int main(){
    int n;
    n = read_test_case(sequence);
    printf("%d\n", longest_increasing_subsequence(n, sequence, dp));
    return 0;
}
