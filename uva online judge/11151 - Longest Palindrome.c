#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#define MAX_SIZE  999
#define BUFFER_SIZE  MAX_SIZE + 2
#define MAX(a,b) (((a)>(b))?(a):(b))

char str[BUFFER_SIZE];
int length[MAX_SIZE][MAX_SIZE];

int longest_polindrom(int n, char str[], int length[][MAX_SIZE]){
    int i, l, max_length = 1;
    if(n == 0)
        return 0;
    for(i = 0; i < n; i++)
        length[i][i] = 1;
    for(i = 0; i < n - 1; i++){
        if(str[i] == str[i + 1]){
            length[i][i + 1] = 2;
            max_length = 2;
        }else{
            length[i][i + 1] = 1;
        }
    }
    for(l = 3; l <= n; l++){
        for(i = 0; i < n - l + 1; i++){
            if(str[i] == str[i + l - 1]){
                length[i][i + l - 1] = length[i + 1][i + l - 2] + 2;
            }else{
                length[i][i + l - 1] = MAX(length[i + 1][i + l - 1], length[i][i + l - 2]);
            }
            max_length = MAX(max_length, length[i][i + l - 1]);
        }
    }
    return max_length;
}

int main(){
    int i, n;
    fgets(str, BUFFER_SIZE, stdin);
    i = atoi(str);
    for(; i > 0; i--){
        fgets(str, BUFFER_SIZE, stdin);
        n = strlen(str) - 1;
        printf("%d\n", longest_polindrom(n, str, length));
    }
    return 0;
}