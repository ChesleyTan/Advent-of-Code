import sys
import hashlib

def has_three(s, c):
    return (c * 3) in s

def has_five(s, c):
    return (c * 5) in s

def first_triple(s, alpha):
    triples = []
    for c in alpha:
        if has_three(s, c):
            triples.append((c, s.index(c * 3)))
    triples = sorted(triples, key=lambda x: x[1])
    return triples[0][0] if triples else None

if __name__ == "__main__":
    i = 0
    hashes = []
    alpha = "abcdef0123456789"
    salt = "yjdafjpo"
    while i < 50000:
        if i % 1000 == 0:
            print i
        md5 = hashlib.md5()
        md5.update(salt + str(i))
        h = md5.hexdigest()
        for _ in xrange(2016):
            md5 = hashlib.md5()
            md5.update(h)
            h = md5.hexdigest()
        hashes.append(h)
        i += 1
    keys = []
    for (i, h) in enumerate(hashes):
        c = first_triple(h, alpha)
        if c:
            for j in xrange(i+1, i + 1001):
                if has_five(hashes[j], c):
                    keys.append((i, j))
                    if len(keys) == 64:
                        print keys
                        sys.exit(0)
                    break


