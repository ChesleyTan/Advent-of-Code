#!/usr/bin/python
import hashlib

salt="bgvyzdsv"
i = 1
while True:
    m = hashlib.md5()
    m.update(salt + str(i))
    h = m.hexdigest()
    if h[:5] == '00000':
        print i
        break
    i += 1
