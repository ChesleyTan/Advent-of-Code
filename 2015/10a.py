#!/usr/bin/python
def look_and_say(n):
    out = ""
    i = 0
    while i < len(n):
        c = n[i]
        runlength = 0
        while i < len(n) and n[i] == c:
            i += 1
            runlength += 1
        out += str(runlength) + c
    return out

ans = "1113122113"
for i in xrange(40):
    ans = look_and_say(ans)
print len(ans)
