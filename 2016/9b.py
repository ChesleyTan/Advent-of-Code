def expand(s):
    i = 0
    l = 0
    while i < len(s):
        c = s[i]
        if c == "(":
            end = s.index(")", i)
            marker = s[i+1:end]
            tokens = marker.split("x")
            length = int(tokens[0])
            rep = int(tokens[1])
            substring = s[end+1:end+1+length]
            l += expand(substring) * rep
            i = end + 1 + length
        else:
            i += 1
            l += 1
    return l

if __name__ == "__main__":
    with open("9a.input", 'r') as f:
        lines = f.read().strip().split('\n')
        for line in lines:
            print expand(line)
