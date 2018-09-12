#include<stdio.h>
#include<stdlib.h>
#define MAX_CHAMBERS 5
#define MAX_SPECIMENT (MAX_CHAMBERS * 2)
#define ABS(x) ((x) < 0 ? - (x) : (x))

int compare_int_desc(const void* a, const void* b){
    if( *(int*)a == *(int*)b ) 
        return 0;
    return *(int*)a < *(int*)b ? 1 : -1;
}

int read_test_case(int *n, int *m, int masses[]){
    int i;
    if(scanf("%d %d", n, m) != 2)
        return 0;
    for(i = 0; i < *m; i++)
        scanf("%d", &masses[i]);
    return 1;
}

void load_balancing(int n, int m, int masses[], int chambers[][2]){
    int i;
    for(i = 0; i < m; i++){
        if(i < n)
            chambers[i][0] = masses[i];
        else
            chambers[2 * n - i  - 1][1] = masses[i];
    }
}

double imbalance(int n, int chambers[][2]){
    int i;
    double am = 0, imb = 0;
    for(i = 0; i < n; i++)
        am += chambers[i][0] + chambers[i][1];
    am = am / n;
    for(i = 0; i < n; i++)
        imb += ABS(chambers[i][0] + chambers[i][1] - am);
    return imb;
}

int main(){
    int i = 1, j, n, m, masses[MAX_SPECIMENT], chambers[MAX_CHAMBERS][2];
    while(read_test_case(&n, &m, masses)){
        for(j = 0; j < n; j++)
            chambers[j][0] = chambers[j][1] = 0;
        qsort(masses, m, sizeof(int), compare_int_desc);
        load_balancing(n, m, masses, chambers);
        printf("Set #%d\n", i);
        for(j = 0; j < n; j++){
            printf(" %d:", j);
            if(chambers[j][0] > 0)
                printf(" %d", chambers[j][0]);
            if(chambers[j][1] > 0)
                printf(" %d", chambers[j][1]);
            printf("\n");
        }
        printf("IMBALANCE = %.5lf\n\n", imbalance(n, chambers));
        i++;
    }
    return 0;
}
