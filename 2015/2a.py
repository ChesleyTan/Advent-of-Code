#!/usr/bin/python

data = open('2a.txt', 'r').read().split('\n')[:-1]
total = 0
for line in data:
    (l, w, h) = map(int, line.split('x'))
    (s1, s2, s3) = (l*w, w*h, h*l)
    total += 2*s1 + 2*s2 + 2*s3 + min(s1, s2, s3)
print total
