import sys
import hashlib
def in_bounds(x, y):
    valid = [
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 0),
                (1, 1),
                (1, 2),
                (1, 3),
                (2, 0),
                (2, 1),
                (2, 2),
                (2, 3),
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3),
            ]
    return (x, y) in valid

def door_open(direction, h):
    md5 = hashlib.md5()
    md5.update(h)
    hashval = md5.hexdigest()
    openvals = ["b", "c", "d", "e", "f"]
    if direction == "U":
        return hashval[0] in openvals
    elif direction == "D":
        return hashval[1] in openvals
    elif direction == "L":
        return hashval[2] in openvals
    elif direction == "R":
        return hashval[3] in openvals

def neighboring_rooms(x, y, h):
    up = (x, y-1)
    down = (x, y+1)
    left = (x-1, y)
    right = (x+1, y)
    neighbors = []
    if in_bounds(*up) and door_open("U", h):
        neighbors.append((up, h + "U"))
    if in_bounds(*down) and door_open("D", h):
        neighbors.append((down, h + "D"))
    if in_bounds(*left) and door_open("L", h):
        neighbors.append((left, h + "L"))
    if in_bounds(*right) and door_open("R", h):
        neighbors.append((right, h + "R"))
    return neighbors

if __name__ == "__main__":
    h = "gdjjyniy"
    frontier = neighboring_rooms(0, 0, h)
    path = ""
    while len(frontier) > 0:
        (node, node_hash) = frontier.pop()
        if node == (3, 3):
            if len(node_hash) > len(path):
                path = node_hash
                print len(path) - len(h)
            continue
        neighbors = neighboring_rooms(node[0], node[1], node_hash)
        #print node, neighbors
        frontier.extend(neighbors)
    print path

