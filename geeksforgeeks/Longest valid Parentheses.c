#include <stdio.h>
#include <string.h>
#define MAX_N 100000

char string[MAX_N + 2];
int stack[MAX_N], closed[MAX_N];

int longest_valid_parentheses(int n, char string[], int stack[], int closed[]){
    int i, j, length = 0, max_length = 0, stack_size = 0;
    for(i = 0; i < n; i++){
        closed[i] = 0;
        if(string[i] == '('){
            stack[stack_size++] = i;
        }else if(stack_size > 0){
            j = stack[--stack_size];
            closed[i] = closed[j] = 1;
        }
    }
    for(i = 0; i < n; i++){
        if(closed[i]){
            length += 1;
            if(length > max_length)
                max_length = length;
        }else{
            length = 0;
        }
    }
    return max_length;
}

int main(){    
    int t, n;
    scanf("%d", &t);
    while(t-- > 0){
        scanf("%s", string);
        n = (int) strlen(string);
        printf("%d\n", longest_valid_parentheses(n, string, stack, closed));
    }
    return 0;
}
