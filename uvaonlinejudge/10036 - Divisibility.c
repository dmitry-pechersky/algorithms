#include<stdio.h>
#include<string.h>
#define ABS(x) ((x)<0 ? -(x) : (x))
#define MAX_N 10000
#define MAX_K 100

int nums[MAX_N];

void read_test_case(int *n, int *k, int nums[]){
    int i;
    scanf("%d %d", n, k);
    for(i = 0; i < *n; i++){
        scanf("%d", &nums[i]);
        nums[i] = nums[i] % *k;
    }
}

int divisibility_dp(int n, int k, int nums[]){
    int i, j;
    char array1[MAX_K] = {1}, array2[MAX_K];
    char *current =  array1, *previous = array2, *tmp;
    for(i = 0; i < n; i++){
        tmp = current; current = previous; previous = tmp;
        memset(current, 0, k);
        for(j = 0; j < k; j++){
            if(previous[j]){
                current[ABS(j + nums[i]) % k] = 1;
                current[ABS(j - nums[i]) % k] = 1;
            }
        }
    }
    return current[0];
}

int main(){
    int t, n, k;
    scanf("%d", &t);
    while(t-- > 0){
        read_test_case(&n, &k, nums);
        printf(divisibility_dp(n, k, nums) ? "Divisible\n": "Not divisible\n") ;
    }
    return 0;
}