#include <stdio.h>

int main(){
    int a, b;
    unsigned int c;
    while(scanf("%d", &a) == 1){
        c = (unsigned int) a;
        b = ((255 & c) << 24) | (((255 << 8) & c) << 8) | (((255 << 16) & c) >> 8) | (((255 << 24) & c) >> 24);
        printf("%d converts to %d\n", a, b);
    }
    return 0;
}