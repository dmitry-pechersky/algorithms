#include <stdio.h>
#include <math.h>
#include <limits.h>
#define MAX_COMPUTERS 8
#define MAX_SETS 256
#define INF INT_MAX

struct Point{
    int x;
    int y;
};

struct Point computers[MAX_COMPUTERS];

int order[MAX_COMPUTERS];

double adj_matrix[MAX_COMPUTERS][MAX_COMPUTERS], tsp[MAX_COMPUTERS][MAX_SETS];

int read_test_case(struct Point computers[]){
    int n, i;
    scanf("%d", &n);
    for(i = 0; i < n; i++)
        scanf("%d %d", &computers[i].x, &computers[i].y);
    return n;
}

double distance(struct Point a, struct Point b){
    double x = a.x - b.x, y = a.y - b.y;
    return sqrt(x * x + y * y);
}

void build_adjustment_matrix(int n, struct Point computers[], double adj_matrix[][MAX_COMPUTERS]){
    int i, j;
    for(i = 0; i < n; i++)
        for(j = i + 1; j < n; j++)
            adj_matrix[i][j] = adj_matrix[j][i] = distance(computers[i], computers[j]) + 16;
}

void traveling_salesman(int n, double adj_matrix[][MAX_COMPUTERS], double tsp[][MAX_SETS], int order[]){
    int s, i, j, k, s_minus_i;
    for(s = 1; s < pow(2, n); s++){
        for(i = 0; i < n; i++){
            if((1 << i & s) == 0){
                tsp[i][s] = INF;
            }else if(1 << i == s){
                tsp[i][s] = 0;
            }else{
                s_minus_i = s ^ 1 << i;
                tsp[i][s] = INF;
                for(j = 0; j < n; j++)
                    if(tsp[j][s_minus_i] < INF && tsp[j][s_minus_i] + adj_matrix[i][j] < tsp[i][s])
                        tsp[i][s] = tsp[j][s_minus_i] + adj_matrix[i][j];
            }
        }
    }
    s = pow(2, n) - 1;
    i = 0;
    for(j = 1; j < n ; j++)
        if(tsp[j][s] < tsp[i][s])
            i = j;
    for(k = 0; k < n - 1; k++){
        order[k] = i;
        s_minus_i = s ^ 1 << i;
        for(j = 0; j < n; j++){
            if(tsp[j][s_minus_i] < INF && tsp[j][s_minus_i] + adj_matrix[i][j] == tsp[i][s]){
                i = j;
                break;
            }
        }
        s = s_minus_i;
    }
    order[n - 1] = i;
}

int main(){
    int n, i = 1, j;
    float total_feet;
    while((n = read_test_case(computers))){
        build_adjustment_matrix(n, computers, adj_matrix);
        printf("**********************************************************\n");
        printf("Network #%d\n", i++);
        traveling_salesman(n, adj_matrix, tsp, order);
        total_feet = 0;
        for(j = 0; j < n - 1; j++){
            printf("Cable requirement to connect (%d,%d) to (%d,%d) is %.2f feet.\n", 
                computers[order[j]].x, computers[order[j]].y,
                computers[order[j + 1]].x, computers[order[j + 1]].y,
                adj_matrix[order[j]][order[j + 1]]);
                total_feet += adj_matrix[order[j]][order[j + 1]];
        }
        printf("Number of feet of cable required is %.2f.\n", total_feet);
    }
    return 0;
}
