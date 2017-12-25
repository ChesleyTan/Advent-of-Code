from collections import defaultdict
counts = [defaultdict(int) for _ in range(8)]

if __name__ == "__main__":
    with open("6a.input", 'r') as f:
        lines = f.read().strip().split('\n')
        for line in lines:
            for (i, c) in enumerate(line):
                counts[i][c] += 1
        print "".join([max(count.keys(), key=lambda k: count[k]) for count in counts])
