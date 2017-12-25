#!/usr/bin/python
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
containers = []
for line in data:
    containers.append(int(line))
containers = sorted(containers, key=lambda x: -x)
#containers = [20, 15, 10, 5, 5]
min_length_combination = 1e100
num_min_length_combinations = 0
def make_combination(i, desired, combination=[], total=0):
    global min_length_combination, num_min_length_combinations
    if total == desired:
        if len(combination) < min_length_combination:
            min_length_combination = len(combination)
            num_min_length_combinations = 1
            print combination
        elif len(combination) == min_length_combination:
            num_min_length_combinations += 1
            print combination
        return
    if i >= len(containers) or len(combination) > min_length_combination:
        return
    if containers[i] + total <= desired:
        new_combo = list(combination)
        new_combo.append(containers[i])
        make_combination(i+1, desired, combination=new_combo, total=total+containers[i])
    make_combination(i+1, desired, combination=combination, total=total)
print containers
#make_combination(0, 25)
make_combination(0, 150)
print min_length_combination, num_min_length_combinations
