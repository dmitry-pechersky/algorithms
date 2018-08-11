from collections import deque

MAX_ELEMENTS = 999999

class TeamQueue:
    def __init__(self, team_n):
        self.teams = [deque() for i in range(team_n)]
        self.queue = deque()
    
    def enqueue(self, team, value):
        if len(self.teams[team]) == 0:
            self.queue.append(team)
        self.teams[team].append(value)

    def dequeue(self):
        team = self.queue[0]
        value = self.teams[team].popleft()
        if len(self.teams[team]) == 0:
            self.queue.popleft()
        return value

def read_team_number():
    return int(input())

def read_team():
    return (int(i) for i in input().split()[1:])

def read_command():
    line = input().split()
    if line[0] == 'STOP':
        return None
    return line[0], (int(line[1]) if line[0] == 'ENQUEUE' else None)

if __name__ == '__main__':
    membership = [None] * MAX_ELEMENTS
    for i, team_n in enumerate(iter(read_team_number, 0), start=1):
        print('Scenario #{}'.format(i))
        team_queue = TeamQueue(team_n)
        for team in range(team_n):
            for element in read_team():
                membership[element] = team
        for command, element in iter(read_command, None):
            if command == 'ENQUEUE':
                team_queue.enqueue(membership[element], element)
            else:
                print(team_queue.dequeue())
        print()
