import sys
if __name__ == "__main__":
    password = "abcdefgh"
    with open("21a.input", 'r') as f:
        lines = f.read().strip().split('\n')
        for line in lines:
            tokens = line.split()
            if tokens[0] == "swap":
                if tokens[1] == "position":
                    idx1 = int(tokens[2])
                    idx2 = int(tokens[5])
                    password = list(password)
                    tmp = password[idx1]
                    password[idx1] = password[idx2]
                    password[idx2] = tmp
                    password = "".join(password)
                elif tokens[1] == "letter":
                    letter1 = tokens[2]
                    letter2 = tokens[5]
                    tmp = ""
                    for c in password:
                        if c == letter1:
                            tmp += letter2
                        elif c == letter2:
                            tmp += letter1
                        else:
                            tmp += c
                    password = tmp
            elif tokens[0] == "reverse":
                idx1 = int(tokens[2])
                idx2 = int(tokens[4])
                password = password[:idx1] + password[idx1:idx2+1][::-1] + password[idx2+1:]
            elif tokens[0] == "move":
                idx1 = int(tokens[2])
                idx2 = int(tokens[5])
                password = list(password)
                tmp = password.pop(idx1)
                password.insert(idx2, tmp)
                password = "".join(password)
            elif tokens[0] == "rotate":
                if tokens[1] == "left":
                    steps = int(tokens[2])
                    steps %= len(password)
                    password = password[steps:] +  password[:steps]
                elif tokens[1] == "right":
                    steps = int(tokens[2])
                    steps %= len(password)
                    password = password[-steps:] + password[:-steps]
                elif tokens[1] == "based":
                    letter = tokens[6]
                    steps = password.index(letter)
                    if steps >= 4:
                        steps += 1
                    steps += 1
                    steps %= len(password)
                    password = password[-steps:] + password[:-steps]
    print password

