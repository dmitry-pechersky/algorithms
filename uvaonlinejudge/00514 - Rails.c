#include <stdio.h>
#define MAX_SIZE 1000

int permutation[MAX_SIZE], stack[MAX_SIZE];

int read_permutation(int n, int permutation[]){
    int i;
    for(i = 0; i < n; i++){
        scanf("%d", &permutation[i]);
        if(i == 0 && permutation[0] == 0)
            return 0;
    }
    return 1;
}

int is_possible(int n, int permutation[], int stack[]){
    int i, coach = 1, stack_size = 0;
    for(i = 0; i < n; i++){
        while(1){
            if(stack_size > 0 && stack[stack_size - 1] == permutation[i]){
                stack_size--;
                break;
            }
            if(coach <= n){
                stack[stack_size++] = coach;
                coach++;
            }else{
                return 0;
            }
        }
    }
    return 1;
}

int main(){
    int n;
    while(scanf("%d", &n) && n){
        while(read_permutation(n, permutation)){
            printf("%s\n", is_possible(n, permutation, stack) ? "Yes" : "No");
        }
        printf("\n");
    }
    return 0;
}
