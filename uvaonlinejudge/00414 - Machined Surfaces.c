#include <stdio.h>
#define MAX_ROWS 12
#define COLUMNS_CNT 25

char buffer[MAX_ROWS][COLUMNS_CNT];

int read_test_case(char buffer[][COLUMNS_CNT]){
    int i, j, n;
    scanf("%d", &n);
    for(i = 0; i < n; i++)
        scanf("%*[\n]%25c", buffer[i]);
    return n;
}

int main(){
    int i, j, n, total, min, cnt;
    while(n = read_test_case(buffer)){
        total = 0; min = COLUMNS_CNT;
        for(i = 0; i < n; i++){
            cnt = 0;
            for(j = 0; j < COLUMNS_CNT; j++)
                if(buffer[i][j] == ' ')
                    cnt++;
            min = cnt < min ? cnt : min;
            total += cnt;
        }
        printf("%d\n", total - min * n);
    }
    return 0;
}
