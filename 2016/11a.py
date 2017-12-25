import sys
from itertools import combinations

# NOTE: Incomplete solution: rather should use a BFS rather than recursion, and
# hash states so that chips are mutually interchangeable

#state = (0, [
#            [ "PG", "P" ],
#            [ "CG", "QG", "RG", "LG"],
#            [ "C", "Q", "R", "L"],
#            []
#        ])
state = (0, [
            [ "P" ],
            [ ],
            [ ],
            []
        ])

def hash_state(s):
    h = str(s[0])
    for i in range(4):
        level = "".join(sorted(s[1][i]))
        h += level + "."
    return h

MAX_STEPS = 100
INVALID = 1e99

def deepcopy(l):
    return [list(x) for x in l]

def is_chip(item):
    return not "G" in item

def is_generator(item):
    return not is_chip(item)

def check_state(s):
    for level in s[1]:
        for item in level:
            if is_chip(item):
                if (item + "G") in level:
                    continue
                else:
                    # Invalid if another generator is on the level
                    for item in level:
                        if is_generator(item):
                            return False
    return True

cache = {}

def num_steps_to_finish(s, steps):
    s_hash = hash_state(s)
    if s_hash in cache:
        return cache[s_hash]
    if steps > MAX_STEPS:
        cache[s_hash] = INVALID
        return INVALID
    if check_state(s) and len(s[1][3]) == 1:
        if s_hash in cache:
            cache[s_hash] = min(cache[s_hash], steps)
        else:
            cache[s_hash] = steps
        return steps
    elif not check_state(s):
        cache[s_hash] = INVALID
        return INVALID
    # TODO enumerate all next states
    # recursively call on next state
    # Take 1 item
    items = [x[0] for x in combinations(s[1][s[0]], 1)]
    other_levels = [x for x in range(4) if x != s[0] and abs(x - s[0]) <= 1]
    min_steps = INVALID
    for next_level in other_levels:
        print next_level
        for item in items:
            print(item)
            new_state = (next_level, deepcopy(s[1]))
            print(s[0], new_state)
            new_state[1][s[0]].remove(item)
            new_state[1][next_level].append(item)
            min_steps = min(min_steps, num_steps_to_finish(new_state, steps + 1))
    # Take 2 items
    if s_hash in cache:
        cache[s_hash] = min(cache[s_hash], min_steps)
    else:
        cache[s_hash] = min_steps
    return min_steps

if __name__ == "__main__":
    print num_steps_to_finish(state, 0)
