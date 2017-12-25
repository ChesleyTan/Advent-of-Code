#!/usr/bin/python

data = open(__file__[:-3]+ '.txt', 'r').read()
visited = set()
santas = [[0,0],[0,0]]
i = 0
for c in data:
    visited.add((santas[i][0],santas[i][1]))
    if c == '<':
        santas[i][0] -= 1
    elif c == '>':
        santas[i][0] += 1
    elif c == '^':
        santas[i][1] += 1
    elif c == 'v':
        santas[i][1] -= 1
    i = (i + 1) % 2
print len(visited)
