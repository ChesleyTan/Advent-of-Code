import re

def process_line(l):
    in_brackets = False
    babs = set()
    for i in xrange(0, len(l) - 2):
        c = l[i]
        if c == '[':
            in_brackets = True
        elif c == ']':
            in_brackets = False
        else:
            if c == l[i+2] and c != l[i+1]:
                if not in_brackets:
                    babs.add(c + l[i+1] + l[i+2])
    for i in xrange(0, len(l) - 2):
        c = l[i]
        if c == '[':
            in_brackets = True
        elif c == ']':
            in_brackets = False
        else:
            if c == l[i+2] and c != l[i+1]:
                if in_brackets:
                    if (l[i+1] + c + l[i+1]) in babs:
                        return True
    return False

total = 0
if __name__ == "__main__":
    with open("7b.input", 'r') as f:
        lines = f.read().strip().split('\n')
        for line in lines:
            if process_line(line):
                total += 1
    print(total)
