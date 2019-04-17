def read_test_case():
    while True:
        cards = input().split()
        if len(cards) > 1:
            cards.extend(input().split())
            yield cards
        else: 
            return

if __name__ == '__main__':
    for cards in read_test_case():
        left, right = [],  [[card] for card in reversed(cards)]
        while len(right) > 0:
            if len(left) > 2 and (left[-3][-1][0] == right[-1][-1][0] or left[-3][-1][1] == right[-1][-1][1]):
                card = right[-1].pop()
                if len(right[-1]) == 0:
                    right.pop()
                left[-3].append(card)
                for i in range(3):
                    right.append(left.pop())
            elif len(left) > 0 and (left[-1][-1][0] == right[-1][-1][0] or left[-1][-1][1] == right[-1][-1][1]):
                card = right[-1].pop()
                if len(right[-1]) == 0:
                    right.pop()
                left[-1].append(card)
                right.append(left.pop())
            else:
                left.append(right.pop())
        print('{} piles remaining:'.format(len(left)), *(len(pile) for pile in left))
