def read_test_cases():
    for i in range(int(input())):
        yield int(input()), [int(i) for i in input().split()], [int(i) for i in input().split()]

if __name__ == '__main__':
    for n, projection1, projection2 in read_test_cases():
        min_blocks, max_blocks = 0, 0
        for i in range(n):
            for j in range(n):
                max_blocks += min(projection1[i], projection2[j])
        projection1_used, projection2_used = [False] * n, [False] * n
        for i in range(n):
            for j in range(n):
                if not projection1_used[i] and not projection2_used[j] and projection1[i] == projection2[j]:
                    projection1_used[i], projection2_used[j] = True, True
                    min_blocks += projection1[i]
        for i in range(n):
            for j in range(n):
                if not projection1_used[i] and  projection1[i] <= projection2[j]:
                    projection1_used[i] = True
                    min_blocks += projection1[i]
                elif not projection2_used[j] and  projection2[j] <= projection1[i]:
                    projection2_used[j] = True
                    min_blocks += projection2[j]
        print("Matty needs at least {} blocks, and can add at most {} extra blocks.".format(min_blocks, max_blocks - min_blocks))
