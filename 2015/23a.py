#!/usr/bin/python
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
instructions = []
for line in data:
    line = line.replace(',', '')
    line = line.split(' ')
    instructions.append(line)
print instructions
ip = 0
regs = {'a': 0, 'b': 0}
while 0 <= ip < len(instructions):
    ins = instructions[ip][0]
    if ins == 'hlf':
        regs[instructions[ip][1]] /= 2
    elif ins == 'tpl':
        regs[instructions[ip][1]] *= 3
    elif ins == 'inc':
        regs[instructions[ip][1]] += 1
    elif ins == 'jmp':
        ip += int(instructions[ip][1])
        continue
    elif ins == 'jie':
        if regs[instructions[ip][1]] % 2 == 0:
            ip += int(instructions[ip][2])
            continue
    elif ins == 'jio':
        if regs[instructions[ip][1]] == 1:
            ip += int(instructions[ip][2])
            continue
    ip += 1
print regs['b']
