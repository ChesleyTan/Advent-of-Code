#!/usr/bin/python
from itertools import combinations
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
weapons = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)]
armors = [(0, 0, 0), (13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)]
ringsets = [(0, 0, 0), (25, 1, 0), (50, 2, 0), (100, 3, 0), (20, 0, 1), (40, 0, 2), (80, 0, 3)]
def determine_win(hp, atk, arm, boss_hp, boss_atk, boss_arm):
    dmg = max(atk - boss_arm, 1)
    boss_dmg = max(boss_atk - arm, 1)
    turns_to_player_win = boss_hp / dmg + min(1, boss_hp % dmg)
    turns_to_boss_win = hp / boss_dmg + min(1, hp % boss_dmg)
    return turns_to_player_win <= turns_to_boss_win
boss_hp = int(data[0].split(': ')[1])
boss_atk = int(data[1].split(': ')[1])
boss_arm = int(data[2].split(': ')[1])
hp = 100
ringsets.extend([(x[0][0] + x[1][0], x[0][1] + x[1][1], x[0][2] + x[1][2]) for x in combinations(ringsets, r=2)])
print "Possible combinations: " + str(len(weapons) * len(armors) * len(ringsets))
min_cost = 1e100
for weapon in weapons:
    for armor in armors:
        for ringset in ringsets:
            cost = weapon[0] + armor[0] + ringset[0]
            atk = weapon[1] + armor[1] + ringset[1]
            arm = weapon[2] + armor[2] + ringset[2]
            if cost < min_cost and determine_win(hp, atk, arm, boss_hp, boss_atk, boss_arm):
                min_cost = cost
print min_cost
