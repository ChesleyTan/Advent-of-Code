def rotc(c, n):
    n %= 26
    offset = ord(c) - ord('a')
    offset += n
    offset %= 26
    return chr(ord('a') + offset)

if __name__ == "__main__":
    with open("4a.input", 'r') as f:
        lines = f.read().strip().split('\n')
        for line in lines:
            checksum = line[line.rindex('[')+1:line.rindex(']')]
            sector_id = int(line[line.rindex('-')+1:line.rindex('[')])
            name = line[:line.rindex('-')]
            words = name.split('-')
            words = map(lambda word: map(lambda c: rotc(c, sector_id), word), words)
            words = map(lambda word: ''.join(word), words)
            fullname = " ".join(words)
            if "pole" in fullname:
                print(fullname, sector_id)
