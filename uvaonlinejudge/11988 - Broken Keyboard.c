#include <stdio.h>
#include <string.h>
#include <assert.h>

#define MAX_LENGTH 100000

struct Node{
    char value;
    struct Node * next;
};

struct List{
    struct Node *head;
    struct Node *tail;
    struct Node *current;
    struct Node *free_nodes;
    int free_nodes_cnt;
};

void list_init(struct List * list, struct Node *free_nodes, int free_nodes_cnt){
    list->head = NULL;
    list->tail = NULL;
    list->current = NULL;
    list->free_nodes = free_nodes;
    list->free_nodes_cnt = free_nodes_cnt;
}

struct Node * list_free_node(struct List *list){
    assert(list->free_nodes_cnt > 0);
    list->free_nodes_cnt--;
    return list->free_nodes + (list->free_nodes_cnt);
}

void list_add(struct List *list, char value){
    struct Node *node = list_free_node(list);
    node->value = value;
    if(list->head == NULL){
        node->next = NULL;
        list->head = list->tail = list->current = node;
    }else if(list->current == NULL){
        node->next = list->head;
        list->current = list->head = node;
    }else{
        node->next = list->current->next;
        list->current = list->current->next = node;
        if(node->next == NULL){
            list->tail = node;
        }
    }
}

void list_to_head(struct List *list){
    list->current = NULL;
}

void list_to_tail(struct List *list){
    list->current = list->tail;
}

void list_print(struct List *list){
    struct Node *node = list->head;
    while(node != NULL){
        printf("%c", node->value);
        node = node->next;
    }
    printf("\n");
}

int main(){
    char buffer[MAX_LENGTH + 2];
    int str_len, i;
    struct Node nodes[MAX_LENGTH];
    struct List list;
    while(fgets(buffer, sizeof(buffer), stdin) && (str_len = strlen(buffer) - 1) > 0){
        list_init(&list, nodes, MAX_LENGTH);
        for(i=0; i<str_len; i++){
            if(buffer[i] == '['){
                list_to_head(&list);
            }else if(buffer[i] == ']'){
                list_to_tail(&list);
            }else{
                list_add(&list, buffer[i]);
            }
        }
        list_print(&list);
    }    
    return 0;
}