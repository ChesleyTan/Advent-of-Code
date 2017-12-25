pos_x = 1
pos_y = 1
numpad = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
         ]
def process_cmd(c):
    global pos_x, pos_y
    if c == 'U':
        pos_y = max(0, pos_y - 1)
    elif c == 'D':
        pos_y = min(2, pos_y + 1)
    elif c == 'L':
        pos_x = max(0, pos_x - 1)
    elif c == 'R':
        pos_x = min(2, pos_x + 1)

if __name__ == "__main__":
    with open("2a.input", 'r') as f:
        lines = f.read().strip().split('\n')
        for line in lines:
            for cmd in line:
                process_cmd(cmd)
            print numpad[pos_y][pos_x]
