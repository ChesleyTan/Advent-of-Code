#!/usr/bin/python

data = open('2b.txt', 'r').read().split('\n')[:-1]
total = 0
for line in data:
    (l, w, h) = map(int, line.split('x'))
    (p1, p2, p3) = (2*l+2*w, 2*w+2*h, 2*h+2*l)
    total += min(p1, p2, p3) + l * w * h
print total
