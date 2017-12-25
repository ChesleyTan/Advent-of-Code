import sys
import hashlib

def has_trap(row, idx):
    if idx < 0 or idx >= len(row):
        return False
    return row[idx] == "^"

def get_new_row(prev_row):
    new_row = ""
    for idx in xrange(len(prev_row)):
        (left, center, right) = (has_trap(prev_row, idx-1),
                                 has_trap(prev_row, idx),
                                 has_trap(prev_row, idx+1))
        trap_patterns = [
                (True, True, False),
                (False, True, True),
                (True, False, False),
                (False, False, True)
        ]
        new_row += "^" if (left, center, right) in trap_patterns else "."
    return new_row

if __name__ == "__main__":
    last_row = ".^^^^^.^^.^^^.^...^..^^.^.^..^^^^^^^^^^..^...^^.^..^^^^..^^^^...^.^.^^^^^^^^....^..^^^^^^.^^^.^^^.^^"
    rows = [last_row]
    while len(rows) < 400000:
        last_row = get_new_row(last_row)
        rows.append(last_row)
    print sum(map(lambda row: sum(map(lambda x: 1 if x == "." else 0, row)), rows))

