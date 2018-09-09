#include <stdio.h>
#include <math.h>
#define MAX_N 8
#define MAX_NUMS 100

int cache[MAX_N / 2][MAX_NUMS];

void quirksome_squares(int n, int cache[][MAX_NUMS]){
    int i = 0, i2 = 0, max_i = pow(10,  n) - 1, ten_pow_half_n = pow(10, n / 2);
    while(i2 <= max_i){
        if(i2 % ten_pow_half_n + i2 / ten_pow_half_n == i) 
            cache[n / 2 - 1][++cache[n / 2 - 1][0]] = i2;
        i++;
        i2 = i * i;
    }
}

int main(){
    int i, n;
    while(scanf("%d", &n) == 1){
        if(cache[n / 2 - 1][0] == 0)
            quirksome_squares(n, cache);
        for(i = 1; i <= cache[n / 2 - 1][0]; i++)
            printf("%0*d\n", n, cache[n / 2 - 1][i]);
    }
    return 0;
}
