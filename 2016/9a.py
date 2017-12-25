def expand(s):
    res = ""
    i = 0
    while i < len(s):
        c = s[i]
        if c == "(":
            end = s.index(")", i)
            marker = s[i+1:end]
            tokens = marker.split("x")
            length = int(tokens[0])
            rep = int(tokens[1])
            substring = s[end+1:end+1+length]
            res += substring * rep
            i = end + 1 + length
        else:
            res += c
            i += 1
    return res

if __name__ == "__main__":
    with open("9a.input", 'r') as f:
        lines = f.read().strip().split('\n')
        for line in lines:
            print expand(line)
            print len(expand(line))
        print(expand("X(8x2)(3x3)ABCY"))
