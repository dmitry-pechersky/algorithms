import sys

if __name__ == '__main__':
    for line in iter(sys.stdin.readline, ''):
        a = int(line)
        b = (0xff & a) << 24 | (0xff00 & a) << 8 | (0xff0000 & a) >> 8 | (0xff000000 & a) >> 24
        if b & (1 << 31) :
            b = 4294967295 ^ b ^ (-1)
        print('{} converts to {}'.format(a, b))
