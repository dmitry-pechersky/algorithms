#include <stdio.h>
#include <string.h>
#include <assert.h>

#define  MAX_LENGTH 1024

struct Stack{
    char array[MAX_LENGTH * 2];
    int size;
    int max_size;
};

void stack_init(struct Stack *stack){
    stack->size = 0;
    stack->max_size = sizeof(stack->array);
}

void stack_push(struct Stack *stack, char value){
    assert(stack->size < stack->max_size);
    stack->array[stack->size] = value;
    stack->size++;
}

char stack_pop(struct Stack *stack){
    assert(stack->size > 0);
    stack->size--;
    return stack->array[stack->size];
}

int stack_is_empty(struct Stack *stack){
    return stack->size == 0;
}

int stack_top(struct Stack *stack){
    assert(stack->size > 0);
    return stack->array[stack->size - 1];
}

void stack_print(struct Stack *stack){
    int i;
    for(i=0; i<stack->size; i++){
        if(i > 0){
            printf(" %c", stack->array[i]);
        }else{
            printf("%c", stack->array[i]);
        }
    }
    printf("\n");
}

void anagrams(char *str1, char *str2,int str_len, int str1_i, int str2_i, struct Stack *str_stack, struct Stack *oper_stack){
    if(str1_i < str_len){
        stack_push(str_stack, str1[str1_i]);
        stack_push(oper_stack, 'i');
        anagrams(str1, str2, str_len, str1_i + 1, str2_i, str_stack, oper_stack);
        stack_pop(str_stack);
        stack_pop(oper_stack);
    }
    if(!stack_is_empty(str_stack) && str2[str2_i] == stack_top(str_stack)){
        stack_pop(str_stack);
        stack_push(oper_stack, 'o');
        anagrams(str1, str2, str_len, str1_i, str2_i + 1, str_stack, oper_stack);
        stack_push(str_stack, str2[str2_i]);
        stack_pop(oper_stack);
    }
    if(str1_i >= str_len && str2_i >= str_len){
        stack_print(oper_stack);
    }
}

int main(){
    char str1[MAX_LENGTH + 2], str2[MAX_LENGTH + 2];
    int str1_len, str2_len;

    struct Stack str_stack, oper_stack;

    while(fgets(str1, sizeof(str1), stdin) && (str1_len = strlen(str1) - 1) > 0){
        fgets(str2, sizeof(str2), stdin);
        str2_len = strlen(str2) - 1;
        stack_init(&str_stack);
        stack_init(&oper_stack);
        printf("[\n");
        if(str1_len == str2_len){
            anagrams(str1, str2, str1_len, 0, 0, &str_stack, &oper_stack);
        }
        printf("]\n");
    }
    return 0;
}