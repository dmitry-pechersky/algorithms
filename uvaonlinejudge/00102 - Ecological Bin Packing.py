import sys
import math

def recycling(bottels):
    bin_collor, min_moves = None, math.inf
    total_bottels = sum(bottels)
    colors = (0, 2, 1)
    for i in colors:
        for j in colors:
            for k in colors:
                if i != j and j != k and i !=k:
                    moves = total_bottels - bottels[i] - bottels[3 + j] - bottels[6 + k]
                    if moves < min_moves:
                        min_moves = moves
                        bin_collor = (i, j, k)
    return bin_collor, min_moves

if __name__ == '__main__':
    colors = ['B', 'G', 'C']
    while True:
        bottels = [int(i) for i in sys.stdin.readline().split()]
        if len(bottels) == 0:
            break
        bin_color, moves = recycling(bottels)
        print('{}{}{} {}'.format(colors[bin_color[0]], colors[bin_color[1]], colors[bin_color[2]], moves))
        
