import sys

regs = {'a':0, 'b':0, 'c':0, 'd': 0}
if __name__ == "__main__":
    with open("12a.input", 'r') as f:
        lines = f.read().strip().split('\n')
        ip = 0
        while ip < len(lines):
            cmd = lines[ip]
            tokens = cmd.split()
            if tokens[0] == "cpy":
                if tokens[1] in regs:
                    regs[tokens[2]] = regs[tokens[1]]
                else:
                    regs[tokens[2]] = int(tokens[1])
            elif tokens[0] == "inc":
                regs[tokens[1]] += 1
            elif tokens[0] == "dec":
                regs[tokens[1]] -= 1
            elif tokens[0] == "jnz":
                if (tokens[1] in regs and regs[tokens[1]] != 0) or\
                   (tokens[1] not in regs and int(tokens[1]) != 0):
                    ip += int(tokens[2])
                    continue
            ip += 1
        print regs
