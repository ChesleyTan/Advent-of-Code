import sys

position = (0, 0)
direction = (0, 1)
past_positions = set()
def process_direction(d):
    global direction
    if d == 'L':
        # counter-clockwise 90 transformation
        direction = (-direction[1], direction[0])
    elif d == 'R':
        # clockwise 90 transformation
        direction = (direction[1], -direction[0])

def move(n):
    global position
    for _ in range(n):
        position = (position[0] + direction[0], position[1] + direction[1])
        if position in past_positions:
            print(position)
            print(abs(position[0]) + abs(position[1]))
            sys.exit(0)
        else:
            past_positions.add(position)

if __name__ == "__main__":
    with open("1b.input", 'r') as f:
        cmds = f.read().strip().split(', ')
        for cmd in cmds:
            d = cmd[0]
            n = int(cmd[1:])
            process_direction(d)
            move(n)
    print(position)
    print(abs(position[0]) + abs(position[1]))
