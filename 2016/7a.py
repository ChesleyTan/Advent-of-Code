import re

def process_line(l):
    in_brackets = False
    has_tls = False
    for i in xrange(0, len(l) - 3):
        c = l[i]
        if c == '[':
            in_brackets = True
        elif c == ']':
            in_brackets = False
        else:
            if l[i+1] == l[i+2] and c == l[i+3] and c != l[i+1]:
                if not in_brackets:
                    has_tls = True
                else:
                    return False
    return has_tls

total = 0
if __name__ == "__main__":
    with open("7a.input", 'r') as f:
        lines = f.read().strip().split('\n')
        for line in lines:
            if process_line(line):
                total += 1
    print(total)
