valid = 0

if __name__ == "__main__":
    with open("3b.input", 'r') as f:
        lines = f.read().strip().split('\n')
        lines = map(lambda l: map(int, l.strip().split()), lines)
        for r in xrange(0, len(lines), 3):
            for c in range(3):
                lengths = [lines[r][c], lines[r+1][c], lines[r+2][c]]
                total = sum(lengths)
                passes = 0
                for length in lengths:
                    if length < total - length:
                        passes += 1
                if passes == 3:
                    valid += 1
    print(valid)
