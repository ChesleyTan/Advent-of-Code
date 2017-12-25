#!/usr/bin/python

data = open(__file__[:-3]+ '.txt', 'r').read()
visited = set()
(x, y) = (0, 0)
for c in data:
    visited.add((x,y))
    if c == '<':
        x -= 1
    elif c == '>':
        x += 1
    elif c == '^':
        y += 1
    elif c == 'v':
        y -= 1
print len(visited)
