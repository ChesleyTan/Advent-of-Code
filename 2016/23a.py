import sys

def toggle(cmd):
    tokens = cmd.split()
    if tokens[0] == "cpy":
        tokens[0] = "jnz"
    elif tokens[0] == "inc":
        tokens[0] = "dec"
    elif tokens[0] == "dec":
        tokens[0] = "inc"
    elif tokens[0] == "jnz":
        tokens[0] = "cpy"
    elif tokens[0] == "tgl":
        tokens[0] = "inc"
    return " ".join(tokens)

regs = {'a':7, 'b':0, 'c':0, 'd': 0}
if __name__ == "__main__":
    with open("23.input", 'r') as f:
        lines = f.read().strip().split('\n')
        ip = 0
        while ip < len(lines):
            try:
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
                        if tokens[2] in regs:
                            offset = regs[tokens[2]]
                        else:
                            offset = int(tokens[2])
                        ip += offset
                        continue
                elif tokens[0] == "tgl":
                    if tokens[1] in regs:
                        offset = regs[tokens[1]]
                    else:
                        offset = int(tokens[1])
                    target = ip + offset
                    if target < 0 or target >= len(lines):
                        pass
                    else:
                        lines[target] = toggle(lines[target])
                ip += 1
                print lines
                print regs
            except ValueError:
                continue
        print regs
