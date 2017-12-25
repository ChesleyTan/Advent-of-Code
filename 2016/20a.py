import sys
def optimize_ranges(ranges):
    new_ranges = []
    i = 0
    while i < len(ranges) - 1:
        (start1, end1) = ranges[i]
        (start2, end2) = ranges[i+1]
        if start2 <= end1 <= end2:
            new_ranges.append((start1, end2))
            i += 2
        elif end1 == start2 - 1:
            new_ranges.append((start1, end2))
            i += 2
        elif start1 <= start2 <= end1 and start1 <= end2 <= end1:
            new_ranges.append((start1, end1))
            i += 2
        else:
            new_ranges.append((start1, end1))
            i += 1
    if i == len(ranges) - 1:
        new_ranges.append(ranges[i])
    return new_ranges

if __name__ == "__main__":
    ranges = []
    with open("20a.input", 'r') as f:
        lines = f.read().strip().split('\n')
        for line in lines:
            tokens = line.split('-')
            start = int(tokens[0])
            end = int(tokens[1])
            ranges.append((start, end))
        ranges = sorted(ranges, key=lambda (rstart, rend): rstart)
    new_ranges = optimize_ranges(ranges)
    while len(new_ranges) < len(ranges):
        ranges = new_ranges
        new_ranges = optimize_ranges(ranges)
    print(len(ranges))
    print(ranges)

