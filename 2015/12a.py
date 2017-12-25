#!/usr/bin/python
import re
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
print sum(map(int, re.findall(r'-?[0-9]+', data[0])))
