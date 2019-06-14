import sys

if __name__ == '__main__':
    words = [chr(i) for i in range(97, 123)]
    start = 0
    for i in range(4):
        next_start = len(words)
        for word in words[start:]:
            for char in range(ord(word[-1]) + 1, 123):
                words.append(word + chr(char))
        start = next_start
    dic = dict(zip(words, (i for i in range(1,len(words) + 1))))
    for line in (line.strip() for line in sys.stdin):
        print(dic.get(line, 0))
