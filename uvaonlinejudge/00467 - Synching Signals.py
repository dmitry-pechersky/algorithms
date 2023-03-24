import sys

def synching_signal(signals):
    min_signal = min(signals)
    for time in range(min_signal * 2, 3601):
        is_sync = True
        for signal in signals:
            if time % (signal * 2) >= (signal - 5):
                is_sync = False
                break
        if is_sync:
            return time
    return None

if __name__ == '__main__':
    for i, signals in enumerate(([int(value) for value in row.split()] for row in sys.stdin), 1):  
        time = synching_signal(signals)
        if time is not None:
            print('Set {} synchs again at {} minute(s) and {} second(s) after all turning green.'.format(i, time // 60, time % 60))
        else:
            print('Set {} is unable to synch after one hour.'.format(i))
