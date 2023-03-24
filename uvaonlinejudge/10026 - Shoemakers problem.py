if __name__ == '__main__':
    for i in range(int(input())):
        jobs = []
        _, n = input(), int(input())
        for j in range(n):
            time, fine = (int(k) for k in input().split())
            jobs.append((j + 1, fine / time))
        jobs.sort(key=lambda job: job[1], reverse=True)
        if i > 0:
            print()
        print(*(job[0] for job in jobs))
