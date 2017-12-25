pos_x = 0
pos_y = 2
numpad = [
            ['0', '0', '1', '0', '0'],
            ['0', '2', '3', '4', '0'],
            ['5', '6', '7', '8', '9'],
            ['0', 'A', 'B', 'C', '0'],
            ['0', '0', 'D', '0', '0']
         ]
def process_cmd(c):
    global pos_x, pos_y
    if c == 'U':
        new_pos_y = max(0, pos_y - 1)
        if numpad[new_pos_y][pos_x] == '0':
            return
        else:
            pos_y = new_pos_y
    elif c == 'D':
        new_pos_y = min(4, pos_y + 1)
        if numpad[new_pos_y][pos_x] == '0':
            return
        else:
            pos_y = new_pos_y
    elif c == 'L':
        new_pos_x = max(0, pos_x - 1)
        if numpad[pos_y][new_pos_x] == '0':
            return
        else:
            pos_x = new_pos_x
    elif c == 'R':
        new_pos_x = min(4, pos_x + 1)
        if numpad[pos_y][new_pos_x] == '0':
            return
        else:
            pos_x = new_pos_x

if __name__ == "__main__":
    with open("2b.input", 'r') as f:
        lines = f.read().strip().split('\n')
        for line in lines:
            for cmd in line:
                process_cmd(cmd)
            print numpad[pos_y][pos_x]
