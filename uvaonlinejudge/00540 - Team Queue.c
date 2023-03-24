#include <stdio.h>
#include <assert.h>
#define MAX_ELEMENTS 1000000
#define MAX_TEAMS 1000
#define MAX_ELEMENTS_IN_TEAM 1000
#define MAX_QUEUE_SIZE MAX_ELEMENTS_IN_TEAM

struct Queue{
    int array[MAX_QUEUE_SIZE];
    int size;
    int pos;
};

struct TeamQueue{
    struct Queue teams[MAX_TEAMS];
    struct Queue queue;
};

struct TeamQueue team_queue;

int membership[MAX_ELEMENTS];

void queue_enqueue(struct Queue *queue, int value){
    assert(queue->size < MAX_QUEUE_SIZE);
    queue->array[(queue->pos + queue->size) % MAX_QUEUE_SIZE] = value;
    queue->size++;
}

int queue_dequeue(struct Queue *queue){
    int value;
    assert(queue->size > 0);
    value = queue->array[queue->pos];
    queue->pos = (queue->pos + 1) % MAX_QUEUE_SIZE;
    queue->size--;
    return value;
}

int queue_top(struct Queue *queue){
    assert(queue->size > 0);
    return queue->array[queue->pos];
}

void team_queue_init(struct TeamQueue *team_queue){
    int i;
    for(i = 0; i < MAX_TEAMS; i++)
        team_queue->teams[i].pos = team_queue->teams[i].size = 0;
    team_queue->queue.pos = team_queue->queue.size = 0;
}

void team_queue_enqueue(struct TeamQueue *team_queue, int team, int element){
    if(team_queue->teams[team].size == 0){
        queue_enqueue(&team_queue->queue, team);
    }
    queue_enqueue(&team_queue->teams[team], element);
}

int team_queue_dequeue(struct TeamQueue *team_queue){
    int team, element;
    team = queue_top(&team_queue->queue);
    element = queue_dequeue(&team_queue->teams[team]);
    if(team_queue->teams[team].size == 0)
        queue_dequeue(&team_queue->queue);
    return element;
}

void read_elements(int team_n, int membership[]){
    int team, i, element;
    for(team = 0; team < team_n; team++){
        scanf("%d", &i);
        while(i-- > 0){
            scanf("%d", &element);
            membership[element] = team;
        }
    }
}

int read_command(char *command, int *element){
    char buffer[20];
    scanf("%s", buffer);
    *command = buffer[0];
    if(*command == 'S')
        return 0;
    if(*command == 'E')
        scanf("%d", element);
    return 1;
}

int main(){
    char command;
    int team_n, argument, i = 1;
    while(scanf("%d", &team_n) && team_n){
        printf("Scenario #%d\n", i++);
        read_elements(team_n, membership);
        team_queue_init(&team_queue);
        while(read_command(&command, &argument)){
            if(command == 'E'){
                team_queue_enqueue(&team_queue, membership[argument], argument);
            }else{
                printf("%d\n", team_queue_dequeue(&team_queue));
            }
        }
        printf("\n");
    }
    return 0;
}
