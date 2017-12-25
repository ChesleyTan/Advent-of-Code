#!/usr/bin/python
from collections import defaultdict
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
uniques = set()
subs = defaultdict(list)
for line in data:
    if not line:
        break
    line = line.split(' => ')
    subs[line[0]].append(line[1])
compound = data[-1]
print compound
print subs
element_start = 0
element_end = 1
while element_end <= len(compound):
    if element_end == len(compound) or compound[element_end].isupper(): # end of element reached
        el = compound[element_start:element_end]
        before = compound[:element_start]
        after = compound[element_end:]
        for sub in subs[el]:
            uniques.add(before + sub + after)
        element_start = element_end
    element_end += 1
print uniques
print len(uniques)
