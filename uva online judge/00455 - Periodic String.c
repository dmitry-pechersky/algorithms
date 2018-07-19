#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define MAX_SIZE 80

char str[MAX_SIZE + 2];
int pre_func[MAX_SIZE];

void prefix_function(int n, char str[], int pre_func[]){
    int i, j = 0;
    pre_func[0] = 0;
    for(i = 1; i < n; i++){
        while(j > 0 && str[i] != str[j])
            j = pre_func[j - 1];
        if(str[i] == str[j])
            j++;
        else
            j = 0;
        pre_func[i] = j;
    }
}

int main(){
    int t, i, n, period;
    fgets(str, MAX_SIZE + 2, stdin);
    t = atoi(str);
    while(t-- > 0){
        fgets(str, MAX_SIZE + 2, stdin); 
        fgets(str, MAX_SIZE + 2, stdin);
        n = strlen(str) - 1;
        prefix_function(n, str, pre_func);
        period = (n - pre_func[n - 1]) != 0 && (n % (n - pre_func[n - 1]) == 0) ? (n - pre_func[n - 1]) : n;
        printf("%d\n", period);
        if(t > 0)
            printf("\n");
    }
    return 0;
}
