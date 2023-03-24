import sys

def return_to_initial_positions(positions, piles, after_block):
    pile = positions[after_block]
    while piles[pile][-1] != after_block:
        block = piles[pile].pop()
        piles[block].append(block)
        positions[block] = block

def move_pile(positions, piles, a, b):
    temp_stack = []
    source_pile, target_pile = positions[a], positions[b]
    while True:
        temp_stack.append(piles[source_pile].pop())
        if temp_stack[-1] == a:
            break
    while len(temp_stack) > 0:
        positions[temp_stack[-1]] = target_pile
        piles[target_pile].append(temp_stack.pop())

if __name__ == '__main__':
    n = int(input())
    positions = [i for i in range(n)]
    piles = [[i] for i in range(n)]
    for command in (line.split() for line in sys.stdin):
        if command[0] == 'quit':
            break
        a, b = int(command[1]), int(command[3])
        if a == b or positions[a] == positions[b]:
            continue
        if command[0] == 'move':
            return_to_initial_positions(positions, piles, a)
        if command[2] == 'onto':
            return_to_initial_positions(positions, piles, b)
        move_pile(positions, piles, a, b)
    for i in range(n):
        print('{}:'.format(i), *piles[i])
