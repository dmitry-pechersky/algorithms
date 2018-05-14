#include <stdio.h>
#include <string.h>
#define MAX_LENGTH 25

int dx[] = {1,1,0,-1,-1,-1, 0, 1};
int dy[] = {0,1,1, 1, 0,-1,-1,-1};

void fill(char grid[MAX_LENGTH][MAX_LENGTH], int n, int x, int y){
    int i;
    if(x < 0 || x >= n || y < 0 || y >= n || grid[x][y] != '1')
        return;
    grid[x][y] = '0';
    for(i = 0; i < 8; i++){
        fill(grid, n, x + dx[i], y + dy[i]);
    }
}

int main(){
    int x, y, n, cnt, case_i = 1;
    char grid[MAX_LENGTH][MAX_LENGTH], buffer[MAX_LENGTH + 2];
    while(fgets(buffer, sizeof(buffer), stdin)){
        sscanf(buffer, "%d", &n);
        for(x=0; x<n; x++){
            fgets(buffer, sizeof(buffer), stdin);
            memcpy(grid[x], buffer, sizeof(char) * n);
        }
        cnt = 0;
        for(x = 0; x < n; x++){
            for(y = 0; y < n; y++){
                if(grid[x][y] == '1'){
                    fill(grid, n, x, y);
                    cnt++;
                }
            }
        }
        printf("Image number %d contains %d war eagles.\n", case_i, cnt);
        case_i++;
    }
    return 0;
}