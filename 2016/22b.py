import sys
from collections import defaultdict
class Node(object):
    def __init__(self, x, y, size, used, avail):
        self.x = x
        self.y = y
        self.size = size
        self.used = used
        self.avail = avail
        self.is_movable = True
        self.is_goal = False
        self.is_start = False
        self.is_free = False
    def __repr__(self):
        return "(" + str(self.x) + ", " + str(self.y) + ")"

def print_grid(grid, max_x, max_y):
    for y in xrange(max_y + 1):
        for x in xrange(max_x + 1):
            node = grid[y][x]
            if node.is_goal:
                print "G",
            elif node.is_start:
                print "S",
            elif not node.is_movable:
                print "#",
            elif node.is_free:
                print "_",
            else:
                print ".",
        print ""

def move_payload_left(grid, payload_x, payload_y, max_x, max_y):
    path_to_hole = find_path_to_hole(grid, payload_x - 1, payload_y, max_x, max_y)
    print path_to_hole
    path_to_hole[-1].is_free = False
    steps = len(path_to_hole)
    grid[payload_y][payload_x - 1].is_goal = True
    grid[payload_y][payload_x].is_free = True
    grid[payload_y][payload_x].is_goal = False
    return (steps, (payload_x - 1, payload_y))

def find_path_to_hole(grid, target_x, target_y, max_x, max_y):
    frontier = [(grid[target_y][target_x], [])]
    visited = set()
    while len(frontier) > 0:
        (node, path) = frontier.pop(0)
        path = list(path)
        path.append(node)
        if node.is_free:
            return path
        if node.x - 1 >= 0:
            tmp = grid[node.y][node.x - 1]
            if tmp not in visited and tmp.is_movable and not tmp.is_goal:
                visited.add(tmp)
                frontier.append((tmp, list(path)))
        if node.x + 1 <= max_x:
            tmp = grid[node.y][node.x + 1]
            if tmp not in visited and tmp.is_movable and not tmp.is_goal:
                visited.add(tmp)
                frontier.append((tmp, list(path)))
        if node.y - 1 >= 0:
            tmp = grid[node.y - 1][node.x]
            if tmp not in visited and tmp.is_movable and not tmp.is_goal:
                visited.add(tmp)
                frontier.append((tmp, list(path)))
        if node.y + 1 <= max_y:
            tmp = grid[node.y + 1][node.x]
            if tmp not in visited and tmp.is_movable and not tmp.is_goal:
                visited.add(tmp)
                frontier.append((tmp, list(path)))

if __name__ == "__main__":
    nodes = defaultdict(dict)
    all_nodes = []
    min_size = 1e99
    max_x = 0
    max_y = 0
    with open("22a.input", 'r') as f:
        lines = f.read().strip().split('\n')
        for line in lines[2:]:
            tokens = line.split()
            size = int(tokens[1][:-1])
            used = int(tokens[2][:-1])
            min_size = min(min_size, size)
            avail = int(tokens[3][:-1])
            x = int(tokens[0].split('-')[1][1:])
            max_x = max(max_x, x)
            y = int(tokens[0].split('-')[2][1:])
            max_y = max(max_y, y)
            nodes[y][x] = Node(x, y, size, used, avail)
            all_nodes.append(nodes[y][x])
    for node in all_nodes:
        if node.used > min_size:
            node.is_movable = False
        if node.x == 0 and node.y == 0:
            node.is_start = True
        elif node.x == max_x and node.y == 0:
            node.is_goal = True
        if node.used == 0:
            node.is_free = True
    print_grid(nodes, max_x, max_y)
    payload_x = max_x
    payload_y = 0
    total_steps = 0
    while payload_x != 0:
        (steps, (payload_x, payload_y)) = move_payload_left(nodes, payload_x,
                payload_y, max_x, max_y)
        total_steps += steps
        print_grid(nodes, max_x, max_y)
        print steps
    print total_steps


