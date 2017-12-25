#!/usr/bin/python
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
lights = []
for i in xrange(100):
    lights.append([False] * 100)
for row in xrange(len(data)):
    for col in xrange(len(data[row])):
        if data[row][col] == '#':
            lights[row][col] = True
        else:
            lights[row][col] = False
def is_on(r, c):
    if r < 0 or c < 0 or r >= 100 or c >= 100:
        return False
    else:
        return lights[r][c]
def get_num_neighbors_on(r, c):
    num_on = 0
    if is_on(r-1, c-1):
        num_on += 1
    if is_on(r-1, c):
        num_on += 1
    if is_on(r-1, c+1):
        num_on += 1
    if is_on(r, c-1):
        num_on += 1
    if is_on(r, c+1):
        num_on += 1
    if is_on(r+1, c-1):
        num_on += 1
    if is_on(r+1, c):
        num_on += 1
    if is_on(r+1, c+1):
        num_on += 1
    return num_on
def enable_corners():
    global lights
    lights[0][0] = True
    lights[0][99] = True
    lights[99][0] = True
    lights[99][99] = True
def tick():
    global lights
    enable_corners()
    new_lights = []
    for i in xrange(100):
        new_lights.append([False] * 100)
    for r in xrange(100):
        for c in xrange(100):
            neighbors_on = get_num_neighbors_on(r, c)
            if lights[r][c]:
                if neighbors_on == 2 or neighbors_on == 3:
                    new_lights[r][c] = True
                else:
                    new_lights[r][c] = False
            else:
                if neighbors_on == 3:
                    new_lights[r][c] = True
                else:
                    new_lights[r][c] = False
    lights = new_lights
def count_num_on():
    global lights
    count = 0
    for r in xrange(100):
        for c in xrange(100):
            if lights[r][c]:
                count += 1
    return count
for _ in xrange(100):
    tick()
enable_corners()
print lights
print count_num_on()

