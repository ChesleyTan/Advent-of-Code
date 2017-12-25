screen = []
for _ in xrange(6):
    row = []
    for _ in xrange(50):
        row.append(False)
    screen.append(row)

if __name__ == "__main__":
    with open("8a.input", 'r') as f:
        lines = f.read().strip().split('\n')
        for line in lines:
            tokens = line.split()
            if tokens[0] == "rect":
                dims = tokens[1].split("x")
                width = int(dims[0])
                height = int(dims[1])
                for r in xrange(height):
                    for c in xrange(width):
                        screen[r][c] = True
            elif tokens[0] == "rotate":
                if tokens[1] == "row":
                    row = int(tokens[2][2:])
                    amt = int(tokens[4])
                    new_row = [False] * 50
                    for i in xrange(50):
                        new_row[i] = screen[row][(i - amt + 50) % 50]
                    screen[row] = new_row
                elif tokens[1] == "column":
                    col = int(tokens[2][2:])
                    amt = int(tokens[4])
                    new_col = [False] * 6
                    for i in xrange(6):
                        new_col[i] = screen[(i - amt + 6) % 6][col]
                    for i in xrange(6):
                        screen[i][col] = new_col[i]
    total = 0
    for r in xrange(6):
        for c in xrange(50):
            if screen[r][c]:
                total += 1
    print(total)
