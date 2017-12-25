#!/usr/bin/python
from collections import namedtuple
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
Reindeer = namedtuple("Reindeer", "name speed duration cooldown")
reindeer = []

def calc_dist(speed, duration, cooldown, race_duration):
    dist = race_duration / (duration + cooldown) * speed * duration
    dist += min(race_duration % (duration + cooldown), duration) * speed
    return dist

for line in data:
    line = line.split(' ')
    name = line[0]
    speed = int(line[3])
    duration = int(line[6])
    cooldown = int(line[13])
    r = Reindeer(name=name, speed=speed, duration=duration, cooldown=cooldown)
    reindeer.append(r)

dists = [(r.name, calc_dist(r.speed, r.duration, r.cooldown, 2503)) for r in reindeer]
print dists
print sorted(dists, key=lambda x: -x[1])
