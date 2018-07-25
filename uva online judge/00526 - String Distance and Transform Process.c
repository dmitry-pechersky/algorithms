#include <stdio.h>
#include <string.h>
#define MAX_SIZE 80
#define BUFFER_SIZE MAX_SIZE + 2
#define ALIGNMENT_SIZE MAX_SIZE + 1

struct Operation{
    int type;
    int pos;
    char ch;
};

char str1[BUFFER_SIZE], str2[BUFFER_SIZE];
int alignment[ALIGNMENT_SIZE][ALIGNMENT_SIZE];
struct Operation operations[MAX_SIZE];

int read_test_case(int *n1, char str1[], int *n2, char str2[]){
    if( NULL == fgets(str1, BUFFER_SIZE, stdin))
        return 0;
    fgets(str2, BUFFER_SIZE, stdin);
    *n1 = strlen(str1) - 1;
    *n2 = strlen(str2) - 1;
    return 1;
}

void edit_distance(int n1, char str1[], int n2, char str2[], int alignment[][ALIGNMENT_SIZE], struct Operation operations[]){
    int i, j, oper_i = 0;
    for(i = 0; i < n1 + 1; i++)
        alignment[i][0] = i;
    for(i = 1; i < n2 + 1; i++)
        alignment[0][i] = i;
    for(i = 1; i < n1 + 1; i++){
        for(j = 1; j < n2 + 1; j++){
            alignment[i][j] = alignment[i-1][j] + 1;
            if(alignment[i][j] > alignment[i][j-1] + 1)
                alignment[i][j] = alignment[i][j-1] + 1;
            if(str1[i-1] == str2[j-1] && alignment[i][j] > alignment[i-1][j-1])
                alignment[i][j] = alignment[i-1][j-1];
            if(str1[i-1] != str2[j-1] && alignment[i][j] > alignment[i-1][j-1] + 1)
                alignment[i][j] = alignment[i-1][j-1] + 1;
        }
    }
    i = n1; j = n2;
    while(i > 0 || j > 0){
        if(j > 0 && alignment[i][j] == alignment[i][j-1] + 1){
            operations[oper_i].type = 0;
            operations[oper_i].pos = i;
            operations[oper_i].ch = str2[j-1];
            j--; oper_i++;
        }else if(i > 0 && alignment[i][j] == alignment[i-1][j] + 1){
            operations[oper_i].type = 1;
            operations[oper_i].pos = i - 1;
            i--; oper_i++;
        }else if(i > 0 && j > 0 && alignment[i][j] == alignment[i-1][j-1] + 1){
            operations[oper_i].type = 2;
            operations[oper_i].pos = i - 1;
            operations[oper_i].ch = str2[j-1];
            i--; j--; oper_i++;
        }else{
            i--; j--; 
        }
    }
}

int main(){
    int n1, n2, i = 0, oper_i, bias;
    while(read_test_case(&n1, str1, &n2, str2)){
        edit_distance(n1, str1, n2, str2, alignment, operations);
        if(i > 0)
            printf("\n");
        printf("%d\n", alignment[n1][n2]);
        bias = 0;
        for(oper_i = alignment[n1][n2] - 1; oper_i >= 0; oper_i--){
            printf("%d ", alignment[n1][n2] - oper_i);
            if(operations[oper_i].type == 0){
                printf("Insert %d,%c\n", operations[oper_i].pos + bias + 1, operations[oper_i].ch);
                bias++;
            }else if(operations[oper_i].type == 1){
                printf("Delete %d\n", operations[oper_i].pos + bias + 1);
                bias--;
            }else{
                printf("Replace %d,%c\n", operations[oper_i].pos + bias + 1, operations[oper_i].ch);
            }
        }
        i++;
    }
    return 0;
}