#include<stdio.h>
#include<stdlib.h>
#define MAX_JOBS 1000

struct Job{
    int number;
    double fine_per_day;
};

struct Job jobs[MAX_JOBS];

int job_comp(const void * a, const void * b){
    struct Job *a_job = (struct Job *) a, *b_job = (struct Job *) b;
    int res;
    if(a_job->fine_per_day == b_job->fine_per_day)
        if(a_job->number == b_job->number)
            res = 0;
        else if(a_job->number > b_job->number)
            res = 1;
        else
            res = -1;
    else if(a_job->fine_per_day > b_job->fine_per_day)
        res = -1;
    else 
        res = 1;
    return res;
}

int read_test_case(struct Job jobs[]){
    int i, n, days, fine;
    scanf("%d", &n);
    for(i = 0; i < n; i++){
        scanf("%d %d", &days, &fine);
        jobs[i].number = i + 1;
        jobs[i].fine_per_day = (double) fine / (double) days;
    }
    return n;
}

int main(){
    int i, n, t;
    scanf("%d", &t);
    while(t-- > 0){
        n = read_test_case(jobs);
        qsort(jobs, n, sizeof(struct Job), job_comp);
        for(i = 0; i < n; i++){
            if(i > 0)
                printf(" ");
            printf("%d", jobs[i].number);
        }
        if(t > 0)
            printf("\n\n");
        else
            printf("\n");
    }
    return 0;
}
