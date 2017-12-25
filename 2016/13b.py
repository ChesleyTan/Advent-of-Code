import sys

def is_wall(x, y):
    if x < 0 or y < 0:
        return True
    s = 1364 + x * x + 3 * x + 2 * x * y + y + y * y
    num_ones = sum([1 for x in bin(s)[2:] if x == "1"])
    return num_ones % 2 == 1

def dist_from_start(x, y):
    node = prev[(x, y)]
    path_len = 0
    while node != None:
        node = prev[node]
        path_len += 1
    return path_len

if __name__ == "__main__":
    prev = {(1, 1) : None}
    frontier = [(1, 1)]
    visited = set()
    while len(frontier) > 0:
        node = frontier.pop(0)
        if node in visited:
            continue
        visited.add(node)
        x, y = node
        if dist_from_start(x, y) >= 50:
            continue
        if not is_wall(x-1, y):
            frontier.append((x-1, y))
            if (x-1, y) not in prev:
                prev[(x-1, y)] = (x, y)
        if not is_wall(x+1, y):
            frontier.append((x+1, y))
            if (x+1, y) not in prev:
                prev[(x+1, y)] = (x, y)
        if not is_wall(x, y-1):
            frontier.append((x, y-1))
            if (x, y-1) not in prev:
                prev[(x, y-1)] = (x, y)
        if not is_wall(x, y+1):
            frontier.append((x, y+1))
            if (x, y+1) not in prev:
                prev[(x, y+1)] = (x, y)
    print len(visited)

