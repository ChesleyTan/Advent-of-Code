import hashlib

door = "ffykfhsq"
password = "--------"
if __name__ == "__main__":
    i = 0
    while '-' in password:
        h = hashlib.md5()
        h.update(door + str(i))
        hashval = h.hexdigest()
        if hashval[:5] == "00000":
            try:
                pos = int(hashval[5])
                if pos <= 7 and password[pos] == '-':
                    password = password[:pos] + hashval[6] + password[pos+1:]
                    print(password)
            except ValueError:
                pass
        i += 1
    print(password)
