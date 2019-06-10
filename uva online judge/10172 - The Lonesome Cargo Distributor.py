from collections import deque

def read_test_case():
    n, carrier_capacity, station_capacity = (int(i) for i in input().split())
    return n, carrier_capacity, station_capacity, [deque([int(j) - 1 for j in input().split()][1:]) for i in range(n)]

if __name__ == '__main__':
    for i in range(int(input())):
        n, carrier_capacity, station_capacity, stations = read_test_case()
        j, carrier, minutes, unfinished_cargos = 0, [], 0, sum(len(station) for station in stations)
        while unfinished_cargos > 0:
            while len(carrier) > 0 and (len(stations[j]) < station_capacity or carrier[-1] == j):
                if carrier[-1] == j:
                    unfinished_cargos -= 1
                else:
                    stations[j].append(carrier[-1])
                carrier.pop()
                minutes += 1
            while len(carrier) < carrier_capacity and len(stations[j]) > 0:
                carrier.append(stations[j].popleft())
                minutes += 1
            minutes += 2
            j = (j + 1) % n
        print(minutes - 2)
