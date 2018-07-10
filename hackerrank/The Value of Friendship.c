#include <stdio.h>
#define MAX_NODES 100000
#define MAX_EDGES 200000

struct AdjListNode{
    int vertex;
    struct AdjListNode *next;
};

int visited[MAX_NODES], size_cnts[MAX_NODES + 1], nstack[MAX_NODES];

struct AdjListNode adj_list_node_pool[MAX_EDGES * 2];

struct AdjListNode *adj_list[MAX_NODES];

void read_test_case(int *n, int *m, struct AdjListNode *adj_list[], struct AdjListNode adj_list_node_pool[]){
    int i, v, u, node_idx = 0;
    scanf("%d %d", n, m);
    for(i = 0; i < *n; i++)
        adj_list[i] = NULL;
    for(i = 0; i < *m; i++){
        scanf("%d %d", &v, &u);
        v--; u--;
        adj_list_node_pool[node_idx].vertex = u;  
        adj_list_node_pool[node_idx].next = adj_list[v];
        adj_list[v] = &adj_list_node_pool[node_idx++];

        adj_list_node_pool[node_idx].vertex = v;  
        adj_list_node_pool[node_idx].next = adj_list[u];
        adj_list[u] = &adj_list_node_pool[node_idx++];
    }
}

int dfs(struct AdjListNode *adj_list[], int visited[], int v, int nstack[]){
    int stack_size = 1, u, cnt = 1;
    struct AdjListNode *adj_list_node;
    nstack[0] = v;
    visited[v] = 1;
    while(stack_size > 0){
        v = nstack[--stack_size];
        adj_list_node = adj_list[v];
        while(adj_list_node != NULL){
            u = adj_list_node->vertex;
            if(!visited[u]){
                nstack[stack_size++] = u;
                visited[u] = 1;
                cnt++;
            }
            adj_list_node = adj_list_node->next;
        }
    }
    return cnt;
}

void connected_components(int n, struct AdjListNode *adj_list[], int visited[], int size_cnts[], int nstack[]){
    int v;
    for(v = 0; v < n; v++)
        visited[v] = 0;
    for(v = 0; v < n; v++){
        if(!visited[v]){
            size_cnts[dfs(adj_list, visited, v, nstack)] += 1;
        }
    }
}

int main(){
    int t, n, m, i, size, friendships;
    long long int total, friends;
    scanf("%d", &t);
    for(; t > 0; t--){
        read_test_case(&n, &m, adj_list, adj_list_node_pool);
        for(i = 1; i <= n; i++)
            size_cnts[i] = 0;
        connected_components(n, adj_list, visited, size_cnts, nstack);
        total = friends = friendships = 0;
        for(size = n; size >= 2; size--){
            while(size_cnts[size] > 0){
                for(i = 2; i <= size; i++){
                    friends += i * (i - 1) - (i - 1) * (i - 2);
                    total += friends;
                }
                friendships += size - 1;
                size_cnts[size]--;
            }
        }
        total += (m - friendships) * friends;
        printf("%lld\n", total);
    }
    return 0;
}
