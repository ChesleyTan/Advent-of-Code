def gen_data(d, length):
    while len(d) < length:
        inverted = "".join(map(lambda x: "1" if x == "0" else "0", d[::-1]))
        d = d + "0" + inverted
    return d[:length]

def calc_checksum(d):
    checksum = ""
    for i in xrange(0, len(d) - 1, 2):
        if d[i] == d[i+1]:
            checksum += "1"
        else:
            checksum += "0"
    return checksum

def gen_checksum(d):
    checksum = calc_checksum(d)
    while len(checksum) % 2 == 0:
        checksum = calc_checksum(checksum)
    return checksum

if __name__ == "__main__":
    print gen_checksum(gen_data("10001001100000001", 272))
