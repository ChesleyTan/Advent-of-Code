position = (0, 0)
direction = (0, 1)
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
    position = (position[0] + n * direction[0], position[1] + n * direction[1])

if __name__ == "__main__":
    with open("1a.input", 'r') as f:
        cmds = f.read().strip().split(', ')
        for cmd in cmds:
            d = cmd[0]
            n = int(cmd[1:])
            process_direction(d)
            move(n)
    print(position)
    print(abs(position[0]) + abs(position[1]))
