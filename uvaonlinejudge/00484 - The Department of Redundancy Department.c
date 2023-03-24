#include <stdio.h>
#include <assert.h>
#define HASH_SIZE 28151
#define EMPTY -1

int table[HASH_SIZE], values[HASH_SIZE], order[HASH_SIZE];

int _hash(int key, int i){
    return (((unsigned int) key) % HASH_SIZE + i * (1 + ((unsigned int) key) % (HASH_SIZE - 1))) % HASH_SIZE;
}

int _hash_insert(int table[], int key){
    int i, hash;
    for(i = 0; i < HASH_SIZE; i++){
        hash = _hash(key, i);
        if(table[hash] == EMPTY){
            table[hash] = key;
            return hash;
        }
    }
    return -1;
}

int _hash_search(int table[], int key){
    int i, hash;
    for(i = 0; i < HASH_SIZE; i++){
        hash = _hash(key, i);
        if(table[hash] == key){
            return hash;
        }else if(table[hash] == EMPTY){
            break;
        }
    }
    return -1;
}

int main(){
    int i, j, value, values_cnt = 0;
    for(i = 0; i < HASH_SIZE; i++){
        table[i] = EMPTY;
    }
    while(scanf("%d", &value) > 0){
        j = _hash_search(table, value);
        if(j >= 0){
            values[j] += 1;
        }else{
            j = _hash_insert(table, value);
            assert(j >= 0);
            values[j] = 1;
            order[values_cnt++] = value;
        }
    }
    for(i = 0; i < values_cnt; i++){
        printf("%d %d\n", order[i], values[_hash_search(table, order[i])]);
    }
    return 0;
}