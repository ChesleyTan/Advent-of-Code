from collections import defaultdict

def compare(counts, a, b):
    if counts[a] > counts[b]:
        return 1
    elif counts[a] == counts[b]:
        return ord(b) - ord(a)
    else:
        return -1

total = 0
if __name__ == "__main__":
    with open("4a.input", 'r') as f:
        lines = f.read().strip().split('\n')
        for line in lines:
            counts = defaultdict(int)
            checksum = line[line.rindex('[')+1:line.rindex(']')]
            sector_id = int(line[line.rindex('-')+1:line.rindex('[')])
            name = line[:line.rindex('-')].replace('-', '')
            for c in name:
                counts[c] += 1
            print(name, sector_id, checksum)
            freqs = sorted(counts.keys(), cmp=lambda a, b: -compare(counts, a, b))
            if sorted(freqs[:5]) == sorted(checksum):
                print(freqs[:5])
                total += sector_id
    print(total)
