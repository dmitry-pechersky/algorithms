#include <stdio.h>

int main(){
    int n, a, b, order, pos;
    while(scanf("%d", &n) && n > 0){
        a = b = order = pos = 0;
        while(n >> pos){
            if(n & (1 << pos)){
                a |= (order ^ 1) << pos;
                b |= (order & 1) << pos;
                order ^= 1;
            }            
            pos += 1;
        }
        printf("%d %d\n", a, b);
    }
    return 0;
}
