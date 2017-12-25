#!/usr/bin/python
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
diff = 0

for line in data:
    for c in line:
        if c in ['"', '\\']:
            diff += 1
    diff += 2

print diff
