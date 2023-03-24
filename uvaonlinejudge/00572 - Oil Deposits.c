#include<stdio.h>
#define MAX_SIZE 100
#define BUFFER_SIZE 102
#define STACK_SIZE 20000 

char grid[MAX_SIZE][BUFFER_SIZE];

int stack[STACK_SIZE];

int read_test_case(int *m, int *n, char grid[][BUFFER_SIZE]){
    int i;
    fgets(grid[0], BUFFER_SIZE, stdin);
    sscanf(grid[0], "%d %d", m, n);
    for(i = 0; i < *m; i++)
        fgets(grid[i], BUFFER_SIZE, stdin);
    return *m;
}

void dfs(int m, int n, int x, int y, char grid[][BUFFER_SIZE], int stack[]){
    int stack_size = 0, next_x, next_y;
    grid[x][y] = '*';
    stack[stack_size++] = x; stack[stack_size++] = y;
    while(stack_size > 0){
        y = stack[--stack_size]; x = stack[--stack_size];
        for(next_x = x - 1; next_x <= x + 1; next_x++){
            for(next_y = y - 1; next_y <= y + 1; next_y++){
                if(next_x >= 0 && next_x < m && next_y >= 0 && next_y < n && grid[next_x][next_y] == '@'){
                    grid[next_x][next_y] = '*';
                    stack[stack_size++] = next_x; stack[stack_size++] = next_y;
                }
            }
        }
    }
}

int connected_components(int m, int n, char grid[][BUFFER_SIZE], int stack[]){
    int x, y, cnt = 0;
    for(x = 0; x < m; x++){
        for(y = 0; y < n; y++){
            if(grid[x][y] == '@'){
                cnt += 1;
                dfs(m, n, x, y, grid, stack);
            }
        }
    }
    return cnt;
}

int main(){
    int m, n;
    while(read_test_case(&m, &n, grid)){
        printf("%d\n", connected_components(m, n, grid, stack));
    }
    return 0;
}
