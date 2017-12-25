
import math
import sys
import hashlib

def is_int(f):
    return abs(f - round(f)) < 1e-10

if __name__ == "__main__":
    #for n in range(1, 100):
    #    elves = range(1, n+1)
    #    idx = 0
    #    while len(elves) > 1:
    #        if len(elves) % 10000 == 0:
    #            print len(elves)
    #        target_idx = (idx + len(elves) / 2) % len(elves)
    #        del elves[target_idx]
    #        if target_idx > idx:
    #            idx = (idx + 1) % len(elves)
    #        elif target_idx < idx:
    #            idx %= len(elves)
    #    print n, elves[0]
    n = 3018458
    prev_pow_of_three = 0
    x = 1
    for _ in xrange(n-1):
        if (x == 1 and x > prev_pow_of_three) or (is_int(math.log(x, 3)) and x > prev_pow_of_three):
            print x
            prev_pow_of_three = x
            x = 1
        elif x < prev_pow_of_three:
            x += 1
        else:
            x += 2
    print x

