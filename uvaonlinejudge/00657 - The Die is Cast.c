#include <stdio.h>
#include <string.h>
#define MAX_LENGTH 50

int dx[] = {1, 0, -1, 0};
int dy[] = {0, 1, 0, -1};

void fill_number(char grid[MAX_LENGTH][MAX_LENGTH], int h, int w, int x, int y){
    int i;
    if(x < 0 || x >= h || y < 0 || y >=w || grid[x][y] != 'X')
        return;
    grid[x][y] = '*';
    for(i = 0; i < 4; i++){
        fill_number(grid, h, w, x + dx[i], y + dy[i]);
    }

}

int fill_dice(char grid[MAX_LENGTH][MAX_LENGTH], int h, int w, int x, int y){
    int cnt = 0, i;
    if(x < 0 || x >= h || y < 0 || y >=w || grid[x][y] == '.')
        return 0;
    if(grid[x][y] == 'X'){
        fill_number(grid, h, w, x, y);
        cnt++;
    }
    grid[x][y] = '.';
    for(i = 0; i < 4; i++){
        cnt += fill_dice(grid, h, w, x + dx[i], y + dy[i]);
    }
    return cnt;
}

int main(){
    char grid[MAX_LENGTH][MAX_LENGTH], buffer[MAX_LENGTH + 2];
    int h, w, x, y, cnts[6], is_first, case_i = 1;
    while(1){
        scanf("%d %d\n", &w, &h);
        if(w == 0)
            break;
        for(x = 0; x < h; x++){
            fgets(buffer, sizeof(buffer), stdin);
            memcpy(grid[x], buffer, sizeof(char) * w);            
        }
        memset(cnts, 0, sizeof(cnts));
        for(x = 0; x < h; x++){
            for(y = 0; y < w; y++){
                if(grid[x][y] != '.'){
                    cnts[fill_dice(grid, h, w, x, y)]++;
                }
            }
        }
        printf("Throw %d\n", case_i++);
        is_first = 1;
        for(x = 1; x <= 6; x++){
            for(y = 0; y < cnts[x]; y++){
                if(!is_first)
                    printf(" ");
                printf("%d", x);
                is_first = 0;
            }
        }
        printf("\n\n");
    }
    return 0;
}