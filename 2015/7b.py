#!/usr/bin/python
from collections import namedtuple
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
symtab = {}
rules = []
Rule = namedtuple("Rule", 'cmd args output')
def rshift(val, n): return val>>n if val >= 0 else (val+0x100000000)>>n
def filter_caps(s): return ''.join([c for c in s if c.isupper()])
def eval_constant(s):
    if s.isdigit():
        return int(s)
    elif s in symtab:
        return symtab[s]
    else:
        return 0
def eval_rule(r):
    cmd = r.cmd
    if cmd == 'NOT':
        symtab[r.output] = ~eval_constant(r.args[0])
    elif cmd == 'AND':
        symtab[r.output] = eval_constant(r.args[0]) & eval_constant(r.args[1])
    elif cmd == 'OR':
        symtab[r.output] = eval_constant(r.args[0]) | eval_constant(r.args[1])
    elif cmd == 'LSHIFT':
        symtab[r.output] = eval_constant(r.args[0]) << eval_constant(r.args[1])
    elif cmd == 'RSHIFT':
        symtab[r.output] = rshift(eval_constant(r.args[0]), eval_constant(r.args[1]))
    else:
        symtab[r.output] = symtab[r.args[0]]

for line in data:
    line = line.split(' -> ')
    cmd = filter_caps(line[0])
    if cmd == '':
        if line[0].isdigit():
            prim = eval_constant(line[0])
            symtab[line[1]] = prim
        else:
            r = Rule(cmd=None, args=[line[0]], output=line[1])
            rules.append(r)
    else:
        parts = line[0].split(cmd)
        parts = map(str.strip, parts)
        parts = [_part for _part in parts if _part != '']
        r = Rule(cmd=cmd, args=parts, output=line[1])
        rules.append(r)

def rule_defined(r):
    for arg in r.args:
        if arg not in symtab and not arg.isdigit():
            return False
    return True

print rules
symtab['b'] = 16076
while 'a' not in symtab:
    print symtab
    for rule in rules:
        if rule_defined(rule):
            rules.remove(rule)
            eval_rule(rule)
print symtab['a']
