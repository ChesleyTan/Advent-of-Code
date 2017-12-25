#!/usr/bin/python

target = 33100000
houses = [0] * (target / 10)
for elf in xrange(1, len(houses) + 1):
    for house in xrange(elf, len(houses), elf):
        houses[house] += elf * 10

for house in xrange(len(houses)):
    if houses[house] >= target:
        print house
        break
