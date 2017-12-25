#!/usr/bin/python
from collections import namedtuple
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
rules = {'children': 3
        ,'cats': 7
        ,'samoyeds': 2
        ,'pomeranians': 3
        ,'akitas': 0
        ,'vizslas': 0
        ,'goldfish': 5
        ,'trees': 3
        ,'cars': 2
        ,'perfumes': 1}

for line in data:
    line = line.split(' ')
    i = line[2][:-1]
    ival = int(line[3][:-1])
    j = line[4][:-1]
    jval = int(line[5][:-1])
    k = line[6][:-1]
    kval = int(line[7])
    if rules[i] == ival and rules[j] == jval and rules[k] == kval:
        print line[0] + line[1][:-1]
