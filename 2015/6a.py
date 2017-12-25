#!/usr/bin/python
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
on = set()
off = set()
for x in xrange(1000):
    for y in xrange(1000):
        off.add((x,y))
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
                if (x, y) in on:
                    on.remove((x, y))
                    off.add((x, y))
                else:
                    off.remove((x, y))
                    on.add((x, y))
    elif cmd == 'turn off':
        for x in xrange(x1, x2+1):
            for y in xrange(y1, y2+1):
                off.add((x, y))
                on.discard((x, y))
    elif cmd == 'turn on':
        for x in xrange(x1, x2+1):
            for y in xrange(y1, y2+1):
                on.add((x, y))
                off.discard((x, y))
print len(on)

