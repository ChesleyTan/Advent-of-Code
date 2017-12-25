valid = 0

if __name__ == "__main__":
    with open("3a.input", 'r') as f:
        lines = f.read().strip().split('\n')
        for line in lines:
            lengths = map(int, line.strip().split())
            total = sum(lengths)
            passes = 0
            for length in lengths:
                if length < total - length:
                    passes += 1
            if passes == 3:
                valid += 1
    print(valid)
