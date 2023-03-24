#include<stdio.h>
#include<string.h>
#define MAX_WORDS 100
#define MAX_RULES 1000
#define MAX_LENGTH 258
#define MAX_PASSWORD 256

char words[MAX_WORDS][MAX_LENGTH], rules[MAX_WORDS][MAX_LENGTH];
int password[MAX_PASSWORD];

int read_test_case(int *n, char words[][MAX_LENGTH], int *m, char rules[][MAX_LENGTH]){
    int i;
    if(scanf("%d\n", n) != 1)
        return 0;
    for(i = 0; i < *n; i++){
        fgets(words[i], MAX_LENGTH, stdin);
    }
    scanf("%d\n", m);
    for(i = 0; i < *m; i++){
        fgets(rules[i], MAX_LENGTH, stdin);
    }
    return 1;
}

void print_passwords_rec(int word_len, char word[], int rule_len, char rule[], int i){
    int j;
    if(i < rule_len){
        if(rule[i] == '0'){
            for(j = 0; j < 10; j++){
                password[i] = j;
                print_passwords_rec(word_len, word, rule_len, rule, i + 1);
            }
        }else{
            password[i] = -1;
            print_passwords_rec(word_len, word, rule_len, rule, i + 1);
        }
    }else{
        for(j = 0; j < rule_len; j++){
            if(password[j] == -1){
                printf("%.*s", word_len, word);
            }else{
                printf("%d", password[j]);
            }
        }
        printf("\n");
    }
}

int main(){
    int i, j, n, m, rule_len, word_len;
    while(read_test_case(&n, words, &m, rules)){
        printf("--\n");
        for(i = 0; i < m; i++){
            rule_len = strlen(rules[i]) - 1;
            for(j = 0; j < n; j++){
                word_len = strlen(words[j]) - 1;
                print_passwords_rec(word_len, words[j], rule_len, rules[i], 0);
            }
        }
    }
    return 0;
}
