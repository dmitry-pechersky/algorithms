#include <stdio.h>
#include <string.h>
#define MAX_SIZE 100000
#define BUFFER_SIZE MAX_SIZE * 2 + 3

char buffer[BUFFER_SIZE];
int prefix_func[MAX_SIZE * 2 + 1];

int read_test_case(char buffer[]){
    if(fgets(buffer,MAX_SIZE + 2, stdin) == NULL)
        return 0;
    return strlen(buffer) - 1;
}

void kmp_prefix_function(int n, char str[], int prefix_func[]){
    int i, j = 0;
    prefix_func[0] = 0;
    for(i = 1; i < n; i++){
        while(j > 0 && str[i] != str[j])
            j = prefix_func[j - 1];
        if(str[i] == str[j])
            j++;
        else
            j = 0;
        prefix_func[i] = j;
    }
}

int main(){
    int n, i;
    char *str = buffer + MAX_SIZE + 1;
    while((n = read_test_case(str)) > 0){
        for(i = 0; i < n; i++){
            buffer[MAX_SIZE - 1 - i] = str[i];
        }
        kmp_prefix_function(n * 2 + 1, buffer + MAX_SIZE - n , prefix_func);
        for(i = 0; i < n; i++)
            printf("%c", str[i]);
        for(i = n - prefix_func[n * 2] - 1; i >= 0; i--)
            printf("%c", str[i]);
        printf("\n");
    }
    return 0;
}