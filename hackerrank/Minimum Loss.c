#include<stdio.h>
#include<stdlib.h>
#define N 200000
#define INF 10000000000000000

struct Price{
    long long int price;
    int index;
};

struct Price prices[N];

int read_test_case(struct Price prices[]){
    int i, n; 
    scanf("%d", &n);
    for(i = 0; i < n; i++){
        scanf("%lld", &prices[i].price);
        prices[i].index = i;
    }
    return n;
}

int comp(const void *a, const void *b){
    if(((struct Price *) a)->price < ((struct Price *) b)->price)
        return -1;
    return 1;
}

long long int minimum_loss(int n, struct Price prices[]){
    int i;
    long long loss = INF;
    qsort(prices, n, sizeof(struct Price), comp);
    for(i = 0; i < n - 1; i++){
        if(prices[i + 1].price - prices[i].price < loss && prices[i].index > prices[i + 1].index)
            loss = prices[i + 1].price - prices[i].price;
    }
    return loss;
}

int main(){
    int n;
    n = read_test_case(prices);
    printf("%lld\n", minimum_loss(n, prices));
    return 0;
}
