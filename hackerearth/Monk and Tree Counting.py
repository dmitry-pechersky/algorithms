def largest_smaller(values, start, end, value):
    while start != end:
        mid = (start + end) // 2 + (start + end) % 2
        if values[mid] >= value:
            end = mid - 1
        else:
            start = mid
    return start

def count_not_smaller(values, start, end, value):
    if values[start] >= value:
        return end - start + 1
    if values[end] < value:
        return 0
    return  end - largest_smaller(values, start, end, value) 
    
def count(adj_list, values, node, k):
    triplet_cnt = 0
    ancestor_values = []
    for child in adj_list[node]:
        child_triplet_cnt, child_ancestor_values  = count(adj_list, values, child, k)
        ancestor_values.append(values[child])
        ancestor_values.extend(child_ancestor_values)
        triplet_cnt += child_triplet_cnt
    ancestor_values.sort()
    for i in range(len(ancestor_values) - 1):
        triplet_cnt += count_not_smaller(ancestor_values, i + 1, len(ancestor_values) - 1, k - values[node] - ancestor_values[i])
    return triplet_cnt, ancestor_values

if __name__ == '__main__':
    n, k = map(int, input().split())
    values = [int(a) for a in input().split()]
    adj_list = [[] for i in range(n)]
    for node, parent in enumerate((int(i) - 1 for i in input().split()), 1):
        adj_list[parent].append(node)
    cnt, _ = count(adj_list, values, 0, k)
    print(cnt)
