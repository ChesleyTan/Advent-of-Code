import sys
from itertools import permutations

def bfs(grid, start, end):
    visited = set()
    visited.add(start)
    frontier = [(start, 0)]
    max_x = len(grid[0])
    max_y = len(grid)
    while len(frontier) > 0:
        ((node_x, node_y), steps) = frontier.pop(0)
        if grid[node_y][node_x] == end:
            return steps, (node_x, node_y)
        if node_x - 1 >= 0 and (node_x - 1, node_y) not in visited:
            if grid[node_y][node_x - 1] != '#':
                visited.add((node_x - 1, node_y))
                frontier.append(((node_x - 1, node_y), steps + 1))
        if node_x + 1 < max_x and (node_x + 1, node_y) not in visited:
            if grid[node_y][node_x + 1] != '#':
                visited.add((node_x + 1, node_y))
                frontier.append(((node_x + 1, node_y), steps + 1))
        if node_y - 1 >= 0 and (node_x, node_y - 1) not in visited:
            if grid[node_y - 1][node_x] != '#':
                visited.add((node_x, node_y - 1))
                frontier.append(((node_x, node_y - 1), steps + 1))
        if node_y + 1 < max_y and (node_x, node_y + 1) not in visited:
            if grid[node_y + 1][node_x] != '#':
                visited.add((node_x, node_y + 1))
                frontier.append(((node_x, node_y + 1), steps + 1))
    return 1e99, None

def count_steps(grid, order, start, max_steps):
    steps = 0
    for poi in order:
        distance, end = bfs(grid, start, str(poi))
        steps += distance
        if steps > max_steps:
            return 1e99
        start = end
    return steps

if __name__ == "__main__":
    with open("24.input", 'r') as f:
        lines = f.read().strip().split('\n')
        grid = []
        num_points_of_interest = 0
        start = None
        for (r, line) in enumerate(lines):
            for (c, block) in enumerate(line):
                if ord('0') <= ord(block) <= ord('9'):
                    num_points_of_interest += 1
                    if block == '0':
                        start = (c, r)
            row = list(line)
            grid.append(row)
    visiting_orders = [x for x in permutations(range(1, num_points_of_interest))]

    min_steps = 1e99
    best_order = None
    for order in visiting_orders:
        order = list(order)
        order.append(0)
        steps = count_steps(grid, order, start, min_steps)
        if steps < min_steps:
            print order, steps
            min_steps = steps
            best_order = order
    print min_steps, best_order
        
