#!/usr/bin/python
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
subs = {}
for line in data:
    if not line:
        break
    line = line.split(' => ')
    subs[line[1]] = line[0]
compound = data[-1]
print compound
print subs
def replace_one(needle, haystack, replacement):
    needle_pos = haystack.find(needle)
    return haystack[:needle_pos] + replacement + haystack[needle_pos + len(needle):]
def simplify():
    global compound
    longest_sub = ''
    longest_sub_len = -1e100
    for sub in subs:
        if len(sub) > longest_sub_len:
            if sub in compound:
                longest_sub = sub
                longest_sub_len = len(sub)
    compound = replace_one(longest_sub, compound, subs[longest_sub])
num_steps = 0
while compound != 'e':
    num_steps += 1
    print compound
    simplify()
print compound
print num_steps
