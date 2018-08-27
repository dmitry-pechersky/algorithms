#include <stdio.h>
#define MAX_SIZE 100

int matrix[MAX_SIZE][MAX_SIZE];

int kadane(int n, int array[]){
    int i, s = 0, max_sum = array[0];
    for(i = 0; i < n; i++){
        s = s > 0 ? s + array[i] : array[i];
        if(s > max_sum)
            max_sum = s;
    }
    return max_sum;
}

int maximum_sum_submatrix(int n, int matrix[][MAX_SIZE]){
    int left, right, i, max_subarray_sum, max_sum, array[MAX_SIZE];
    max_sum = matrix[0][0];
    for(left = 0; left < n; left++){
        for(i = 0; i < n; i++)
            array[i] = 0;
        for(right = left; right < n; right++){
            for(i = 0; i < n; i++){
                array[i] += matrix[i][right];
            }
            max_subarray_sum = kadane(n, array);
            if(max_subarray_sum > max_sum)
                max_sum = max_subarray_sum;
        }
    }
    return max_sum;
}

int main(){
    int n, i; 
    scanf("%d", &n);
    for(i = 0; i < n * n; i++){
        scanf("%d", &matrix[i / n][i % n]);
    }
    printf("%d\n", maximum_sum_submatrix(n, matrix));
    return 0;
}