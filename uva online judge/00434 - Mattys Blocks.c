#include <stdio.h>
#define MAX_SIZE 8

int read_test_case(int projection1[], int projection2[]){
    int n, i;
    scanf("%d", &n);
    for(i = 0; i < n; i++)
        scanf("%d", &projection1[i]);
    for(i = 0; i < n; i++)
        scanf("%d", &projection2[i]);
    return n;
}

int main(){
    int t, n, i, j, max_blocks, min_blocks, projection1[MAX_SIZE], projection2[MAX_SIZE], projection1_used[MAX_SIZE], projection2_used[MAX_SIZE];
    for(scanf("%d", &t); t > 0; t--){
        n = read_test_case(projection1, projection2);
        max_blocks = min_blocks = 0;
        for(i = 0; i < n; i++)
            projection1_used[i] = projection2_used[i] = 0;                    
        for(i = 0; i < n; i++){
            for(j = 0; j < n; j++){
                max_blocks += projection1[i] < projection2[j] ? projection1[i] : projection2[j];
                if(! projection1_used[i] && ! projection2_used[j] && projection1[i] == projection2[j]){
                    projection1_used[i] = projection2_used[j] = 1;
                    min_blocks += projection1[i];
                }
            }
        }
        for(i = 0; i < n; i++){
            for(j = 0; j < n; j++){
                if(! projection1_used[i] && projection1[i] <= projection2[j]){
                    projection1_used[i] = 1;
                    min_blocks += projection1[i];
                }else if(! projection2_used[j] && projection1[i] >= projection2[j]){
                    projection2_used[j] = 1;
                    min_blocks += projection2[j];
                }
            }
        }
        printf("Matty needs at least %d blocks, and can add at most %d extra blocks.\n", min_blocks, max_blocks - min_blocks);
    }
    return 0;
}
