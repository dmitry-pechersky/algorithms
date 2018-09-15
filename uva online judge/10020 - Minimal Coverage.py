def read_test_case():
    input()
    m = int(input())
    segments = []
    while True:
        segment = [int(i) for i in input().split()]
        if segment == [0,0]:
            break
        segments.append(segment)
    return m, segments

def interval_covering(m, segments):
    segments.sort()
    i, x, best_i, cover = 0, 0, None, []
    while i < len(segments) and x < m:
        if x < segments[i][1]:
            if x >= segments[i][0]:
                if best_i is None or segments[best_i][1] < segments[i][1]:
                    best_i = i
            else:
                if best_i is not None:
                    cover.append(segments[best_i])
                    x = segments[best_i][1]
                    best_i = None
                    continue
                else:
                    break
        i += 1
    if best_i is not None:
        cover.append(segments[best_i])
        x = segments[best_i][1]
    return [] if x < m else cover

if __name__ == '__main__':
    for i in range(int(input())):
        if i > 0:
            print()
        segments = interval_covering(*read_test_case())
        print(len(segments))
        for a, b in segments:
            print(a, b)
