#include<stdio.h>
#define ROW_N 10
#define COLUMN_N 15

char board[ROW_N][COLUMN_N];
int con_comp[ROW_N][COLUMN_N], columns[COLUMN_N];

void read_test_case(char board[][COLUMN_N]){
    int x, y;
    for(x = ROW_N - 1; x >= 0; x--){
        for(y = 0; y < COLUMN_N; y++){
            scanf(" %c", &board[x][y]);
        }
    }
}

int dfs(char board[][COLUMN_N], int columns[], int x, int y, int con_comp[][COLUMN_N], int idx){
    int i, cnt = 0, stack_n = 2, stack[COLUMN_N * ROW_N * 2] = {x, y}, dx[] = {1, 0, -1, 0}, dy[] = {0, 1, 0, -1}, next_x, next_y;
    while(stack_n > 0){
        y = stack[--stack_n]; x = stack[--stack_n];
        for(i = 0; i < 4; i++){
            next_x = x + dx[i]; next_y = y + dy[i];
            if(next_y >= 0 && next_y < COLUMN_N && columns[next_y] > 0 && next_x >= 0 && next_x < columns[next_y] && 
               con_comp[next_x][next_y] == 0 && board[next_x][next_y] == board[x][y]){
                   con_comp[next_x][next_y] = idx;
                   stack[stack_n++] = next_x; stack[stack_n++] = next_y;
                   cnt += 1;
               }
        }
    }
    return cnt;
}

void connected_components(char board[][COLUMN_N], int columns[], int *largest_x, int *largest_y, int *largest_size, int con_comp[][COLUMN_N]){
    int x, y, idx = 1, size;
    *largest_size = 0;
    for(y = 0; y < COLUMN_N && columns[y] > 0; y++){
        for(x = 0; x < columns[y]; x++){
            con_comp[x][y] = 0;
        }
    }
    for(y = 0; y < COLUMN_N && columns[y] > 0; y++){
        for(x = 0; x < columns[y]; x++){
            if(con_comp[x][y] == 0){
                size = dfs(board, columns, x, y, con_comp, idx);
                if(size > *largest_size){
                    *largest_size = size; *largest_x = x; *largest_y = y;
                }
                idx++;
            }
        }
    }
}

void compress_board(char board[][COLUMN_N], int columns[], int con_comp[][COLUMN_N], int idx){
    int x, y, new_x, new_y = 0;
    for(y = 0; y < COLUMN_N && columns[y] > 0; y++){
        new_x = 0;
        for(x = 0; x < columns[y]; x++){
            if(con_comp[x][y] != idx){
                board[new_x][new_y] = board[x][y];
                new_x++;
            }
        }
        if(new_x > 0){
            columns[new_y] = new_x;
            new_y++;
        }
    }
    for(y = new_y; y < COLUMN_N; y++){
        columns[y] = 0;
    }
}

void game(char board[][COLUMN_N], int columns[], int con_comp[][COLUMN_N]){
    int x, y, size, i = 1, final_score = 0, balls_remaining = ROW_N * COLUMN_N;
    char color;
    for(y = 0; y < COLUMN_N; y++)
        columns[y] = ROW_N;
    while(1){
        connected_components(board, columns, &x, &y, &size, con_comp);
        if(size < 2)
            break;
        color = board[x][y];
        compress_board(board, columns, con_comp, con_comp[x][y]);
        final_score += (size - 2) * (size - 2);
        balls_remaining -= size;
        printf("Move %d at (%d,%d): removed %d balls of color %c, got %d points.\n", i, x + 1, y + 1, size, color, (size - 2) * (size - 2));
        i ++;
    }
    if(balls_remaining == 0)
        final_score += 1000;
    printf("Final score: %d, with %d balls remaining.\n", final_score, balls_remaining);
}

int main(){
    int i, t;
    scanf("%d", &t);
    for(i = 1; i <= t; i++){
        read_test_case(board);
        if(i > 1)
            printf("\n");
        printf("Game %d:\n\n", i);
        game(board, columns, con_comp);
    }
    return 0;
}
