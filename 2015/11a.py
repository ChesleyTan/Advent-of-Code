#!/usr/bin/python

ctoi = lambda c: ord(c) - ord('a')
passwd = "vzbxkghb"
pairs = [2*chr(i+ord('a')) for i in xrange(26)]
print pairs
def rule1(s):
    two_prev = -1
    prev = -1
    s = map(ctoi, s)
    for c in s:
        if two_prev == prev - 1 and prev == c - 1:
            return True
        else:
            two_prev = prev
            prev = c
    return False

def rule2(s):
    for c in s:
        if c in "iol":
            return False
    return True

def rule3(s):
    count = 0
    for pair in pairs:
        if pair in s:
            count += 1
        if count > 1:
            return True
    return False

def inc(passwd):
    passwd = map(ctoi, passwd)
    i = len(passwd) - 1
    passwd[i] += 1
    while passwd[i] > 25:
        passwd[i] = 0
        i -= 1
        passwd[i] += 1
    return ''.join([chr(i + ord('a')) for i in passwd])

while not (rule1(passwd) and rule2(passwd) and rule3(passwd)):
    passwd = inc(passwd)
print passwd
