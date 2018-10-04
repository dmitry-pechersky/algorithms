def read_test_case():
    n, k = (int(i) for i in input().split())
    cnts = [0] * 100001
    for value in (int(i) for i in input().split()):
        cnts[value] += 1
    return k, cnts

def minimum_cover(k, cnts):
    value, transmitter, uncovered_value, cnt = 1, None, None, 0
    while value < len(cnts):
        if cnts[value] > 0:
            if transmitter is not None:
                if value - transmitter > k:
                    transmitter, uncovered_value = None, value
            elif uncovered_value is not None:
                if value - uncovered_value > k:
                    transmitter, uncovered_value = prev_value, None
                    cnt += 1
                    continue
            else:
                uncovered_value = value
            prev_value = value            
        value += 1
    if uncovered_value is not None:
        cnt += 1
    return cnt

if __name__ == '__main__':
    print(minimum_cover(*read_test_case()))
