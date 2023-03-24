#include <stdio.h>
#define MAX_NODES 25
#define MAX_DEGREE 3
#define MAX(x, y) (((x) > (y)) ? (x) : (y))

int adj_list[MAX_NODES][MAX_DEGREE + 1], visited[MAX_NODES][MAX_NODES];

int read_test_case(int *n, int adj_list[][MAX_DEGREE + 1]){
    int i, u, v, m;
    scanf("%d %d", n, &m);
    if(*n == 0)
        return 0;
    for(v = 0; v < *n; v++)
        adj_list[v][0] = 0; 
    for(i = 0; i < m; i++){
        scanf("%d %d", &v, &u);
        adj_list[v][++adj_list[v][0]] = u;
        adj_list[u][++adj_list[u][0]] = v;
    }
    return 1;
}

int longest_path_recursive(int n, int adj_list[][MAX_DEGREE + 1], int v, int visited[][MAX_NODES]){
    int length, max_length = 0, i, u;
    for(i = 1; i <= adj_list[v][0]; i++){
        u = adj_list[v][i];
        if(visited[v][u] == 0){
            visited[v][u] = visited[u][v] = 1;
            length = longest_path_recursive(n, adj_list, u, visited) + 1;
            max_length = MAX(max_length, length);
            visited[v][u] = visited[u][v] = 0;
        }
    }
    return max_length;
}

int longest_path(int n, int adj_list[][MAX_DEGREE + 1], int visited[][MAX_NODES]){
    int v, length, max_length = 0;
    for(v = 0; v < n; v++){
        length = longest_path_recursive(n, adj_list, v, visited);
        max_length = MAX(max_length, length);
    }
    return max_length;
}

int main(){
    int n;
    while(read_test_case(&n, adj_list)){
        printf("%d\n", longest_path(n, adj_list, visited));
    }

}