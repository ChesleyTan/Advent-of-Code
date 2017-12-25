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

regs = {'a':12, 'b':0, 'c':0, 'd': 0}
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
                    # Optimize
                    if tokens[1] in regs and tokens[2] not in regs:
                        # Optimize adding from 1 register to another
                        if int(tokens[2]) == -2:
                            prev1 = lines[ip - 1]
                            prev2 = lines[ip - 2]
                            if prev1 == "dec " + tokens[1]:
                                if prev2.split()[0] == "inc":
                                    print "FADD"
                                    inc_reg = prev2.split()[1]
                                    regs[inc_reg] += regs[tokens[1]]
                                    regs[tokens[1]] = 0
                                    ip += 1
                                    continue
                            elif prev2 == "dec " + tokens[1]:
                                if prev1.split()[0] == "inc":
                                    print "FADD"
                                    inc_reg = prev1.split()[1]
                                    regs[inc_reg] += regs[tokens[1]]
                                    regs[tokens[1]] = 0
                                    ip += 1
                                    continue
                        # Optimize multiplication of two registers
                        if int(tokens[2]) == -5:
                            dec_ins = "dec " + tokens[1]
                            last5_ins = lines[ip-5:ip]
                            if dec_ins in last5_ins:
                                if last5_ins[0].split()[0] == "cpy":
                                    cpy_tokens = last5_ins[0].split()
                                    if cpy_tokens[1] in regs and cpy_tokens[2] in regs:
                                        reg1 = cpy_tokens[1]
                                        reg2 = cpy_tokens[2]
                                        if "dec " + reg2 in last5_ins:
                                            dec_reg = reg2
                                            inc_ins = [ins for ins in last5_ins if ins.split()[0] == "inc"]
                                            if inc_ins:
                                                print "FMULT"
                                                inc_ins = inc_ins[0]
                                                inc_reg = inc_ins.split()[1]
                                                regs[inc_reg] += regs[reg1] * regs[tokens[1]]
                                                regs[reg2] = 0
                                                regs[tokens[1]] = 0
                                                ip += 1
                                                continue


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
                print ip
                print lines
                print regs['a']
            except ValueError:
                continue
        print regs
