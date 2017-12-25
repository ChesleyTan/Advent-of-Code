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

(capacity, durability, flavor, texture, calories) = (0, 0, 0, 0, 0)
def calc_score(capacity, durability, flavor, texture, calories):
    sign = 1
    if capacity < 0 or durability < 0 or flavor < 0 or texture < 0:
        sign = -1
    if capacity == 0:
        capacity = 1
    if durability == 0:
        durability = 1
    if flavor == 0:
        flavor = 1
    if texture == 0:
        texture = 1
    return sign * abs(capacity * durability * flavor * texture)

def calc_score_change(ingrd):
    curr_score = calc_score(capacity, durability, flavor, texture, calories)
    new_score = calc_score(capacity + ingrd.capacity, durability +
            ingrd.durability, flavor + ingrd.flavor, texture + ingrd.texture,
            calories + ingrd.calories)
    return new_score - curr_score

def add_ingredient(ingrd):
    global capacity, durability, flavor, texture, calories
    capacity += ingrd.capacity
    durability += ingrd.durability
    flavor += ingrd.flavor
    texture += ingrd.texture
    calories += ingrd.calories

print ingredients
for _ in xrange(100):
    print capacity, durability, flavor, texture, calories, calc_score(capacity, durability, flavor, texture, calories)
    optimal_ingrd = ingredients[0]
    optimal_ingrd_score_change = calc_score_change(optimal_ingrd)
    print optimal_ingrd, optimal_ingrd_score_change
    for ingrd in ingredients[1:]:
        score_change = calc_score_change(ingrd)
        print ingrd, score_change
        if score_change > optimal_ingrd_score_change:
            optimal_ingrd = ingrd
            optimal_ingrd_score_change = score_change
    add_ingredient(optimal_ingrd)
    print optimal_ingrd
print capacity, durability, flavor, texture, calories, calc_score(capacity, durability, flavor, texture, calories)
