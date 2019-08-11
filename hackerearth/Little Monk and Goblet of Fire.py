from collections import deque

if __name__ == '__main__':
    students = [deque() for i in range(4)]
    schools = deque()
    for i in range(int(input())):
        command = input().split()
        if command[0] == 'E':
            school, student = map(int, command[1:])
            if len(students[school - 1]) == 0:
                schools.append(school)
            students[school - 1].append(student)
        else:
            print(schools[0], students[schools[0] - 1].popleft())
            if len(students[schools[0] - 1]) == 0:
                schools.popleft()

