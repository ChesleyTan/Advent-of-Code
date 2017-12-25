#!/usr/bin/python

def rule1(s):
    num_vowels = 0
    for c in s:
        if c in "aeiou":
            num_vowels += 1
        if num_vowels == 3:
            return True
    return False

def rule2(s):
    prev = ''
    for c in s:
        if prev == c:
            return True
        else:
            prev = c
    return False

def rule3(s):
    return "ab" not in s and "cd" not in s and "pq" not in s and "xy" not in s

data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
count = 0
for s in data:
    if rule1(s) and rule2(s) and rule3(s):
        count += 1
print count
