#!/usr/bin/python
data = open("1a.txt", 'r').read()
print data.count('(') - data.count(')')
