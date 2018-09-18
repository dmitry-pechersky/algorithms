#include<stdio.h>
#define MAX_SIZE 1000

int matrix[MAX_SIZE][MAX_SIZE];

int read_test_case(int *n, int *m, int matrix[][MAX_SIZE]){
    int i, j;
    if(scanf("%d %d", n, m) != 2)
        return 0;
    for(i = 0; i < *n; i++){
        for(j = 0; j < *n; j++){
            scanf("%d", &matrix[i][j]);
            if(i > 0)
                matrix[i][j] += matrix[i - 1][j];
            if(j > 0)
                matrix[i][j] += matrix[i][j - 1];
            if(i > 0 && j > 0)
                matrix[i][j] -= matrix[i - 1][j - 1];
        }
    }
    return 1;
}

int main(){
    int i, j, n, m, value, t = 0, total_sum;
    while(read_test_case(&n, &m, matrix)){
        if(t++ > 0)
           printf("\n");
        total_sum = 0;
        for(i = 0; i < n - m + 1; i++){
            for(j = 0; j < n - m + 1; j++){
                value = matrix[i + m - 1][j + m - 1];
                if(i > 0)
                    value -= matrix[i - 1][j + m - 1];
                if(j > 0)
                    value -= matrix[i + m - 1][j - 1];
                if(i > 0 && j > 0)
                    value += matrix[i - 1][j - 1];
                total_sum += value;
                printf("%d\n", value);
            }
        }
        printf("%d\n", total_sum);
    }
    return 0;
}
