#!/usr/bin/python

data = open('2.txt', 'r').read()
floor = 0
for i in xrange(len(data)):
    if data[i] == '(':
        floor += 1
    else:
        floor -= 1
    if floor == -1:
        print i + 1
        break
