if __name__ == '__main__':
    n, m = (int(i) for i in input().split())
    array = []
    for i in range(m):
        a, b, k = (int(i) for i in input().split())
        array.append((a, k))
        array.append((b + 1,  -k))
    value, total, max_total = 0, 0, 0
    for i, k in sorted(array, key=lambda x: x[0]):
        if value != i:
            if total > max_total:
                max_total = total
            value = i
            total += k
        else:
            total += k
    if total > max_total:
        max_total = total
    print(max_total)


    
