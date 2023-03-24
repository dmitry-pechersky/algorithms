#include <stdio.h>
#define N 26 

int sequence[N * N], trails[N], stack[N];

void read_test_case(int *n, int sequence[]){
    char c;
    *n = 0;
    while(1){
        scanf("%c", &c);
        if(c == '\n')
           break;
        sequence[*n] = (int) c - 65;
        (*n) += 1;
    } 
}

void find(int n, int sequence[], int trails[], int stack[]){
    int i, size;
    for(i = 0; i < N; i++)
        trails[i] = 0;
    stack[0] = sequence[0];
    size = 1;
    i = 1;
    while(size > 0){
        if(stack[size - 1] == sequence[i]){
            size--;
        }else{
            trails[stack[size - 1]]++;
            trails[sequence[i]]++;
            stack[size] = sequence[i];
            size++;
        }
        i ++;
    }
}

int main(){
    int i, j, t, n;
    scanf("%d\n", &t);
    for(i = 1; i <= t; i++){
        printf("Case %d\n", i);
        read_test_case(&n, sequence);
        find(n, sequence, trails, stack);
        for(j = 0; j < N; j++)
            if(trails[j] > 0)
                printf("%c = %d\n", j + 65, trails[j]);
    }
    return 0;
}
