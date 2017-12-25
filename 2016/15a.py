import sys

if __name__ == "__main__":
    with open("15a.input", 'r') as f:
        lines = f.read().strip().split('\n')
        disks = []
        for line in lines:
            tokens = line.split()
            disk_no = int(tokens[1][1:])
            positions = int(tokens[3])
            start = int(tokens[11][:-1])
            disks.append((start + disk_no, positions))
        i = 0
        while True:
            for (offset, positions) in disks:
                if (offset + i) % positions == 0:
                    continue
                else:
                    break
            else:
                print i
                sys.exit(0)
            i += 1


