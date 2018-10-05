#include<stdio.h>
#include<stdlib.h>
#define MAX_TRACKS 1000

struct Track{
    int row;
    int a;
    int b;
};

struct Track tracks[MAX_TRACKS];

void read_test_case(int *n, int *m, int *k, struct Track tracks[]){
    int i;
    scanf("%d %d %d", n, m, k);
    for(i = 0; i < *k; i++)
        scanf("%d %d %d", &tracks[i].row, &tracks[i].a, &tracks[i].b);
}

int comp(const void * a, const void * b){
    struct Track *a_track = (struct Track *) a, *b_track = (struct Track *) b;
    if(a_track->row < b_track->row)
        return -1;
    else if(a_track->row > b_track->row)
        return 1;
    else if(a_track->a < b_track->a)
        return -1;
    else if(a_track->a > b_track->a)
        return 1;
    return 0;
}

long long int cover(int k, struct Track tracks[]){
    int i, current_row = 0, current_b;
    long long int cnt = 0;
    qsort(tracks, k, sizeof(struct Track), comp);
    for(i = 0; i < k; i++){
        if(current_row != tracks[i].row){
            current_row = tracks[i].row;
            current_b = 0;
        }
        if(tracks[i].a <= current_b){
            if(tracks[i].b > current_b){
                cnt += (long long int) (tracks[i].b - current_b);
                current_b = tracks[i].b;
            }
        }else{
            cnt += (long long int) (tracks[i].b - tracks[i].a + 1);
            current_b = tracks[i].b;
        }
    }
    return cnt;
}

int main(){
    int n, m, k;
    read_test_case(&n, &m, &k, tracks);
    printf("%lld\n", ((long long int) n) * ((long long int) m) - cover(k, tracks));
    return 0;
}
