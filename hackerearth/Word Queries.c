#include <stdio.h>
#include <string.h>
#define MAX_NODES 1000000
#define MAX_VALUES 26
#define BUFFER_LENGTH 1000000

struct Trie{
    int adj[MAX_NODES][MAX_VALUES];
    int cnts[MAX_NODES];
    int size;
};

struct Trie trie;

void insert(struct Trie *trie, char string[]){
    int i, node = 0, n = strlen(string);
    for(i = 0; i < n; i++){
        if(trie->adj[node][string[i] - 97] == 0){
            trie->size ++;
            trie->adj[node][string[i] - 97] = trie->size;
        }
        node = trie->adj[node][string[i] - 97];
        trie->cnts[node] ++;
    }
}

int query(struct Trie *trie, char string[]){
    int i, node = 0, n = strlen(string);
    for(i = 0; i < n; i++){
        if(trie->adj[node][string[i] - 97] == 0){
            return 0;
        }
        node = trie->adj[node][string[i] - 97];
    }
    return trie->cnts[node];
}

int main(){
    int i, n, q;
    char buffer[BUFFER_LENGTH];
    scanf("%d %d", &n, &q);
    for(i = 0; i < n; i++){
        scanf("%s", buffer);
        insert(&trie, buffer);
    }
    for(i = 0; i < q; i++){
        scanf("%s", buffer);
        printf("%d\n", query(&trie, buffer));
    }
    return 0;
}
