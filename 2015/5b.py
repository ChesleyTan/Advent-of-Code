#!/usr/bin/python
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
def rule1(s):
    for i in xrange(len(s)):
        bigram = s[i:i+2]
        for j in xrange(i+2, len(s)):
            if s[j:j+2] == bigram:
                return True
    return False

def rule2(s):
    two_prev = ''
    prev = ''
    for c in s:
        if c == two_prev:
            return True
        else:
            two_prev = prev
            prev = c
    return False

count = 0
for s in data:
    if rule1(s) and rule2(s):
        count += 1

print count
