#!/usr/bin/python
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
hp = 50
armor = 0
mana = 500
boss_hp = int(data[0].split(': ')[1])
boss_atk = int(data[1].split(': ')[1])
spells = [{'name': 'Magic Missile', 'mana': 53}, {'name':
        'Drain', 'mana': 73}, {'name': 'Shield', 'mana': 113}, {'name': 'Poison', 'mana': 173}, {'name': 'Recharge',
        'mana': 229}]
cooldowns = [0, 0, 0, 0, 0]

def available_spells(mana, cooldowns):
    available = []
    for index, spell in enumerate(spells):
        if spell['mana'] <= mana and cooldowns[index] <= 0:
            available.append(spell)
    return available

def apply_effect(mana, armor, cooldowns, boss_hp):
    new_mana = mana
    new_armor = armor
    new_cooldowns = list(cooldowns)
    new_boss_hp = boss_hp
    for index, cooldown in enumerate(cooldowns):
        if cooldown != 0:
            if index == 2:
                new_armor = 7
            elif index == 3:
                new_boss_hp -= 3
            elif index == 4:
                new_mana += 101
            new_cooldowns[index] = cooldown - 1
    return new_mana, new_armor, new_cooldowns, new_boss_hp

def cast_spell(hp, mana, cooldowns, boss_hp, spell):
    new_hp = hp
    new_mana = mana - spell['mana']
    new_cooldowns = list(cooldowns)
    new_boss_hp = boss_hp
    if spell['name'] == 'Magic Missile':
        new_boss_hp -= 4
    elif spell['name'] == 'Drain':
        new_boss_hp -= 2
        new_hp += 2
    elif spell['name'] == 'Shield':
        new_cooldowns[2] += 6
    elif spell['name'] == 'Poison':
        new_cooldowns[3] += 6
    elif spell['name'] == 'Recharge':
        new_cooldowns[4] += 5
    return new_hp, new_mana, new_cooldowns, new_boss_hp

min_mana_spent = 1e100

def min_to_win(hp, mana, armor, cooldowns, boss_hp, turn, total_mana_spent=0):
    global boss_atk, min_mana_spent
    if total_mana_spent > min_mana_spent:
        return 1e100
    armor = 0
    mana, armor, cooldowns, boss_hp = apply_effect(mana, armor, cooldowns, boss_hp)
    if boss_hp <= 0:
        return total_mana_spent
    if turn % 2 == 0: # player turn
        available = available_spells(mana, cooldowns)
        if available:
            for spell in available:
                new_hp, new_mana, new_cooldowns, new_boss_hp = cast_spell(hp, mana,
                        cooldowns, boss_hp, spell)
                min_mana_spent = min(min_mana_spent, min_to_win(new_hp, new_mana, armor, new_cooldowns,
                        new_boss_hp, turn+1, total_mana_spent=total_mana_spent +
                        spell['mana']))
            return min_mana_spent
        else:
            return 1e100
    else: # boss turn
        boss_dmg = max(1, boss_atk - armor)
        if hp - boss_dmg <= 0:
            return 1e100
        else:
            return min_to_win(hp - boss_dmg, mana, armor, cooldowns, boss_hp,
                    turn+1, total_mana_spent=total_mana_spent)


print min_to_win(hp, mana, armor, cooldowns, boss_hp, 0)
