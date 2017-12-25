#!/usr/bin/python
import re
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
data = data[0]
nestlevels = []
i = 0
while i < len(data):
    if data[i] == '{':
        nestlevels.append(i)
    elif data[i] == '}':
        start = nestlevels.pop()
        if ':"red"' in data[start:i+1]:
            print data[start:i+1]
            data = data[:start] + data[i+1:]
            i = start
    i += 1

print data
print sum(map(int, re.findall(r'-?[0-9]+', data)))
