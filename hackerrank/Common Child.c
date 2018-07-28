#include<stdio.h>
#include<string.h>
#define MAX_SIZE 5000
#define BUFFER_SIZE (MAX_SIZE + 2)

char str1[BUFFER_SIZE], str2[BUFFER_SIZE];

int previous[MAX_SIZE + 1], current[MAX_SIZE + 1];

int lcs(char str1[], int n1, char str2[], int n2, int previous[], int current[]){
    int i, j, *tmp;
    for(i = 1; i <= n1; i++){
        for(j = 1; j <= n2; j++){
            current[j] = current[j - 1];
            if(current[j] < previous[j])
                current[j] = previous[j];
            if(str1[i - 1] == str2[j -1] && current[j] < previous[j - 1] + 1)
                current[j] = previous[j - 1] + 1;
        }
        tmp = current; current = previous; previous = tmp;
    }
    return previous[n2];
}

int main(){
    int n1, n2;
    fgets(str1, BUFFER_SIZE, stdin);
    fgets(str2, BUFFER_SIZE, stdin);
    n1 = strlen(str1) - 1; 
    n2 = strlen(str2) - 1;
    if(str2[n2] != '\n')
        n2++;
    printf("%d\n", lcs(str1, n1, str2, n2, previous, current));
    return 0;
}
