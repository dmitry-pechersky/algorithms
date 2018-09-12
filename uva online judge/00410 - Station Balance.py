import sys

def read_test_case():
    line = sys.stdin.readline().strip()
    if line == '':
        return None
    n, m = (int(i) for i in line.split())
    masses = [int(i) for i in sys.stdin.readline().split()]
    return n, m, masses

def load_balancing(n, m, masses):
    chambers =[[] for i in range(n)]
    for i in range(m):
        if i < n:
            chambers[i].append(masses[i])
        else:
            chambers[2 * n - i - 1].append(masses[i])
    am = sum(masses) / n
    imbalance = sum(abs(sum(c) - am) for c in chambers)
    return chambers, imbalance

if __name__ == '__main__':
    for i, (n, m, masses) in enumerate(iter(read_test_case, None), 1):
        chambers, imbalance = load_balancing(n, m, sorted(masses, reverse=True))
        print('Set #{}'.format(i))
        for j in range(n):
            print(' {}:'.format(j), *chambers[j])
        print('IMBALANCE = {:.5f}\n'.format(imbalance))
