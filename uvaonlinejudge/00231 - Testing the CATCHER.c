#include<stdio.h>
#define MAX_SIZE 1000000

int array[MAX_SIZE], dp[MAX_SIZE];

int read_test_case(int array[]){
    int i = 0;
    while(scanf("%d", &array[i]) && array[i] != -1)
        i++;
    return i;
}

int largest_smaller(int n, int array[], int value){
    int a = 0, b = n - 1, c;
    while(a != b){
        c = (a + b) / 2;
        if(array[c] >= value)
            a = c + 1;
        else
            b = c;
    }
    return b;
}

int longest_decreasing_subsequence(int n, int array[], int dp[]){
    int i, dp_length = 1;
    dp[0] = array[0];
    for(i = 1; i < n; i++){
        if(array[i] <= dp[dp_length - 1])
            dp[dp_length++] = array[i];
        else
            dp[largest_smaller(dp_length, dp, array[i])] = array[i];
    }
    return dp_length;
}

int main(){
    int i = 1, n;
    while((n = read_test_case(array)) > 0){
        if(i > 1)
            printf("\n");
        printf("Test #%d:\n", i++);
        printf("  maximum possible interceptions: %d\n", longest_decreasing_subsequence(n, array, dp));
    }
    return 0;
}
