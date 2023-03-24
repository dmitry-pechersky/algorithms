#include <stdio.h>
#include <string.h>
#define MAX_SIZE 200

int secret[MAX_SIZE];
char message[MAX_SIZE], buffer1[MAX_SIZE], buffer2[MAX_SIZE];

int read_secret(int *n, int secret[]){
    int i;
    scanf("%d", n);
    if(*n == 0)
        return 0;
    for(i = 0; i < *n; i++){
        scanf("%d", &secret[i]);
    }
    return 1;
}

int read_message(int n, int *k, char message[]){
    int i = 0;
    scanf("%d", k);
    if(*k == 0)
        return 0;
    scanf("%*c");
    while(i < n){
        scanf("%c", &message[i]);
        if(message[i] == '\n')
            break;
        i++;
    }
    for(; i < n; i++)
        message[i] = ' ';
    return 1;
}

void encode(int n, int secret[], char buffer1[], char buffer2[]){
    int i;
    for(i = 0; i < n; i++)
        buffer2[secret[i] - 1] = buffer1[i];
}

void cipher(int n, int secret[], int k, char message[], char buffer1[], char buffer2[]){
    int i;
    char *tmp;
    memcpy(buffer1, message, n);
    i = 1;
    while(i <= k){
        encode(n, secret, buffer1, buffer2);
        tmp = buffer2; buffer2 = buffer1; buffer1 = tmp;
        if(memcmp(message, buffer1, n) == 0)
            i =  (k / i) * i;
        i++;
    }
    for(i = 0; i < n; i++)
        message[i] = buffer1[i];
}

int main(){
    int n, k, i;
    while(read_secret(&n, secret)){
        while(read_message(n, &k, message)){
            cipher(n, secret, k, message, buffer1, buffer2);
            for(i = 0; i < n; i++)
                printf("%c", message[i]);
            printf("\n");
        }
        printf("\n");
    }
    return 0;
}
