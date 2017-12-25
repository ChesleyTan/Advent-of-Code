import sys
from collections import defaultdict
class Node(object):
    def __init__(self, x, y, size, used, avail):
        self.x = x
        self.y = y
        self.size = size
        self.used = used
        self.avail = avail

if __name__ == "__main__":
    nodes = defaultdict(dict)
    all_nodes = []
    max_used = 0
    min_size = 1e99
    with open("22a.input", 'r') as f:
        lines = f.read().strip().split('\n')
        for line in lines[2:]:
            tokens = line.split()
            size = int(tokens[1][:-1])
            used = int(tokens[2][:-1])
            max_used = max(max_used, used)
            min_size = min(min_size, size)
            avail = int(tokens[3][:-1])
            x = int(tokens[0].split('-')[1][1:])
            y = int(tokens[0].split('-')[2][1:])
            nodes[y][x] = Node(x, y, size, used, avail)
            all_nodes.append(nodes[y][x])
    viable = []
    for a in all_nodes:
        for b in all_nodes:
            if a == b:
                continue
            if a.used != 0 and a.used <= b.avail:
                viable.append((a, b))
    print len(viable)


