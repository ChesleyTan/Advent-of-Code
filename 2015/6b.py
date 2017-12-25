#!/usr/bin/python
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
lights = []
for x in xrange(1000):
    lights.append([0] * 1000)
cmd = ""
x1 = 0
x2 = 0
y1 = 0
y2 = 0
for line in data:
    if line[:6] == "toggle":
        cmd = "toggle"
        line = line[7:]
    elif line[:8] == "turn off":
        cmd = "turn off"
        line = line[9:]
    elif line[:7] == "turn on":
        cmd = "turn on"
        line = line[8:]
    line = line.split(' through ')
    (x1, y1) = map(int, line[0].split(','))
    (x2, y2) = map(int, line[1].split(','))
    if cmd == 'toggle':
        for x in xrange(x1, x2+1):
            for y in xrange(y1, y2+1):
                lights[x][y] += 2
    elif cmd == 'turn off':
        for x in xrange(x1, x2+1):
            for y in xrange(y1, y2+1):
                lights[x][y] = max(0, lights[x][y] - 1)
    elif cmd == 'turn on':
        for x in xrange(x1, x2+1):
            for y in xrange(y1, y2+1):
                lights[x][y] += 1

total = 0
for x in xrange(1000):
    for y in xrange(1000):
        total += lights[x][y]

print total
