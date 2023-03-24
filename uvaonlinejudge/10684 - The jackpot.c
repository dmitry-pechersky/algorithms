#include<stdio.h>
#define MAX_NUMBERS 10000

int numbers[MAX_NUMBERS];

int read_test_case(int numbers[]){
    int i, n;
    scanf("%d", &n);
    if(n > 0)
        for(i = 0; i < n; i++)
            scanf("%d", &numbers[i]);
    return n;
}

int max_subarray_sum(int n, int array[]){
    int i, max_sum, current_sum;
    max_sum = current_sum = array[0] > 0 ? array[0] : 0;
    for(i = 1; i < n; i++){
        current_sum = array[i] + current_sum > 0 ? array[i] + current_sum : 0;
        if(current_sum > max_sum)
            max_sum = current_sum;
    }
    return max_sum;
}

int main(){
    int n, max_sum;
    while((n = read_test_case(numbers)) > 0){
        max_sum = max_subarray_sum(n, numbers);
        if(max_sum > 0)
            printf("The maximum winning streak is %d.\n", max_sum);
        else 
            printf("Losing streak.\n");
    }
    return 0;
}
