#!/usr/bin/python
from collections import namedtuple
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
reindeer = []
scores = {}

def step(r):
    if r['flystep'] < r['duration']:
        scores[r['name']][0] += r['speed']
        r['flystep'] += 1
    elif r['reststep'] < r['cooldown']:
        r['reststep'] += 1
    else:
        r['flystep'] = 1
        r['reststep'] = 0
        scores[r['name']][0] += r['speed']

def award_points():
    max_dist = 0
    for score in scores.values():
        max_dist = max(max_dist, score[0])
    for score in scores.values():
        if score[0] >= max_dist:
            score[1] += 1

for line in data:
    line = line.split(' ')
    name = line[0]
    speed = int(line[3])
    duration = int(line[6])
    cooldown = int(line[13])
    r = {'name' : name,
         'speed':speed,
         'duration':duration,
         'cooldown':cooldown,
         'flystep':0,
         'reststep':0}
    reindeer.append(r)
    scores[name] = [0, 0]

for i in xrange(2503):
    for r in reindeer:
        step(r)
    award_points()

print reindeer
print scores
