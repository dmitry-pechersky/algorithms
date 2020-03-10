def largest_increasing_subsequence(seq):
    lengths = [0] * 26
    for num in seq:
        lengths[num] = 1
        for i in range(num):
            if lengths[i] >= lengths[num]:
                lengths[num] = lengths[i] + 1
    return max(lengths)

if __name__ == '__main__':
    for i in range(int(input())):
        seq = [ord(char) - 97 for char in input()]
        print(largest_increasing_subsequence(seq))
