#include <stdio.h>
#include <limits.h>

int read_test_case(int bottels[]){
    int i;
    for(i = 0; i < 9; i++){
        if(scanf("%d", &bottels[i]) != 1)
            return 0;
    }
    return 1;
}

int recycling(int bottels[], int bins[]){
    int i, j, k, total_bottels = 0, min_moves = INT_MAX, moves;
    int color_order[] = {0, 2, 1};
    for(i = 0 ; i < 9; i++)
        total_bottels += bottels[i];
    for(i = 0; i < 3; i++){
        for(j = 0; j < 3; j++){
            for(k = 0; k < 3; k++){
                if(i != j && j != k && i != k){
                    moves = total_bottels - bottels[color_order[i]] - bottels[3 + color_order[j]] - bottels[6 + color_order[k]];
                    if(moves < min_moves){
                        min_moves = moves;
                        bins[0] = color_order[i]; bins[1] = color_order[j]; bins[2] = color_order[k];
                    }
                }
            }
        }
    }
    return min_moves;
}

int main(){
    int bottels[9], bins[3], moves;
    char colors[] = {'B', 'G', 'C'};
    while(read_test_case(bottels)){
        moves = recycling(bottels, bins);
        printf("%c%c%c %d\n", colors[bins[0]], colors[bins[1]], colors[bins[2]], moves);
    }
    return 0;
}
