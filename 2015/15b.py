#!/usr/bin/python
from collections import namedtuple
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
Ingredient = namedtuple("Ingredient", "name capacity durability flavor texture calories")
ingredients = []

for line in data:
    line = line.split(' ')
    name = line[0][:-1]
    capacity = int(line[2][:-1])
    durability = int(line[4][:-1])
    flavor = int(line[6][:-1]) 
    texture = int(line[8][:-1]) 
    calories = int(line[10]) 
    ingredients.append(Ingredient(name=name, capacity=capacity, durability=durability,
        flavor=flavor, texture=texture, calories=calories))

#ingredients = [Ingredient(name="Butterscotch", capacity=-1, durability=-2, flavor=6,
#    texture=3, calories=8), Ingredient(name="Cinnamon", capacity=2,
#        durability=3, flavor=-2, texture=-1, calories=3)]

def calc_score(amts):
    capacity = 0
    for k,v in enumerate(amts):
        capacity += v * ingredients[k].capacity
    durability = 0
    for k,v in enumerate(amts):
        durability += v * ingredients[k].durability
    flavor = 0
    for k,v in enumerate(amts):
        flavor += v * ingredients[k].flavor
    texture = 0
    for k,v in enumerate(amts):
        texture += v * ingredients[k].texture
    if capacity < 0 or durability < 0 or flavor < 0 or texture < 0:
        return 0
    return capacity * durability * flavor * texture

def calc_calories(amts):
    calories = 0
    for k,v in enumerate(amts):
        calories += v * ingredients[k].calories
    return calories

print ingredients
#max_score = 0
#for i in xrange(100):
#    j = 100 - i
#    if calc_calories([i, j]) == 500:
#        print i, j
#        max_score = max(calc_score([i, j]), max_score)
#print max_score
max_score = 0
for i in xrange(100):
    for j in xrange(100 - i):
        for k in xrange(100 - i - j):
            l = 100 - i - j - k
            if calc_calories([i, j, k, l]) == 500:
                print i, j, k, l
                max_score = max(calc_score([i, j, k, l]), max_score)
print max_score

