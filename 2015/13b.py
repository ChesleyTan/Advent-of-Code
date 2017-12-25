#!/usr/bin/python
import re
from itertools import permutations
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
edges = []
for i in xrange(9):
    edges.append([0]*9)
name_to_int_map = {'Alice':0,
                   'Bob':1,
                   'Carol':2,
                   'David':3,
                   'Eric':4,
                   'Frank':5,
                   'George':6,
                   'Mallory':7}
for line in data:
    line = line.split(' ')
    a = line[0]
    b = line[-1][:-1]
    val = int(line[3])
    if line[2] == 'lose':
        val *= -1
    edges[name_to_int_map[a]][name_to_int_map[b]] = val

def calc_happiness(ordering):
    total = 0
    for i in xrange(9):
        total += edges[ordering[i]][ordering[i-1]]
        total += edges[ordering[i]][ordering[(i+1)%9]]
    return total

print edges
max_happiness = 0
for perm in permutations(range(len(edges))):
    print perm
    h = calc_happiness(perm)
    print h
    max_happiness = max(h, max_happiness)
print max_happiness
