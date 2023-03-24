#include<stdio.h>
#include<math.h>
#define MAX_CF 11

double cf[MAX_CF];

int read_test_case(double cf[]){
    int i, n;
    scanf("%d", &n);
    if(n == 0)
        return 0;
    for(i = 0; i <= n; i++)
        scanf("%lf", &cf[i]);
    return n + 1;
}

double f(int n, double cf[], double x){
    int i;
    double res = cf[0];
    for(i = 1; i < n; i++)
        res += cf[i] / pow(1 + x, i);
    return res;
}

double find_irr(int n, double cf[], double a, double b){
    double c;
    if(b - a < 0.0001)
        return b;
    c = (a + b) / 2.0;
    if(f(n, cf, c) > 0)
        return find_irr(n, cf, c, b);
    return find_irr(n, cf, a, c);
}

int main(){
    int n;
    while((n = read_test_case(cf)) > 0){
        printf("%.2f\n", find_irr(n, cf, -1.0, 1000000));
    }
    return 0;
}
