import hashlib

door = "ffykfhsq"
password = ""
if __name__ == "__main__":
    i = 0
    while len(password) < 8:
        h = hashlib.md5()
        h.update(door + str(i))
        hashval = h.hexdigest()
        if hashval[:5] == "00000":
            password += hashval[5]
        i += 1
    print (password)
