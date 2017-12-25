#!/usr/bin/python
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
containers = []
for line in data:
    containers.append(int(line))
containers = sorted(containers, key=lambda x: -x)
#containers = [20, 15, 10, 5, 5]
num_combinations = 0
def make_combination(i, desired, combination=[], total=0):
    global num_combinations
    if total == desired:
        num_combinations += 1
        print combination
        return
    if i >= len(containers):
        return
    if containers[i] + total <= desired:
        new_combo = list(combination)
        new_combo.append(containers[i])
        make_combination(i+1, desired, combination=new_combo, total=total+containers[i])
    make_combination(i+1, desired, combination=combination, total=total)
print containers
#make_combination(0, 25)
make_combination(0, 150)
print num_combinations
