import sys
import hashlib

if __name__ == "__main__":
    n = 3018458
    elves = range(1, n+1)
    while len(elves) > 1:
        #print(elves)
        new_elves = []
        for idx in xrange(0, len(elves), 2):
            new_elves.append(elves[idx])
        if len(elves) % 2 == 1:
            elves = new_elves[1:]
        else:
            elves = new_elves
    print elves[0]

