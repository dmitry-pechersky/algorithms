#include<stdio.h>
#include<stdlib.h>
#define MAX_SEGMENTS 100000

struct Segment{
    int a;
    int b;
};

struct Segment segments[MAX_SEGMENTS + 1], cover[MAX_SEGMENTS];

int segment_comp(const void* a, const void* b){
     return ((struct Segment * ) a)->a  - ((struct Segment * ) b)->a;
}

void read_test_case(int *m, int *n, struct Segment segments[]){
    int i = 0;
    scanf("%d", m);
    while(1){
        scanf("%d %d", &segments[i].a, &segments[i].b);
        if(segments[i].a == 0 && segments[i].b == 0)
            break;
        i++;
    }
    *n = i;
}

int interval_covering(int m, int n, struct Segment segments[], struct Segment cover[]){
    int k = 0, x = 0, i = 0, best_i = -1;
    while(i < n && x < m){
        if(segments[i].b > x){
            if(segments[i].a <= x){
                if(best_i == -1 || segments[best_i].b < segments[i].b)
                    best_i = i;
            }else{
                if(best_i > -1){
                    cover[k++] = segments[best_i];
                    x = segments[best_i].b;
                    best_i = -1;
                    continue;
                }else{
                    break;
                }
            }
        }
        i++;
    }
    if(best_i > -1){
        cover[k++] = segments[best_i];
        x = segments[best_i].b;
    }
    return x < m ? 0 : k;
}

int main(){
    int t, n, m, k, i;
    scanf("%d", &t);
    while(t-- > 0){        
        read_test_case(&m, &n, segments);
        qsort(segments, n, sizeof(struct Segment), segment_comp);
        k = interval_covering(m, n, segments, cover);
        printf("%d\n", k);
        for(i = 0; i < k; i++)
            printf("%d %d\n", cover[i].a, cover[i].b);
        if(t > 0)
            printf("\n");
    }
    return 0;
}
