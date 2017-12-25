#!/usr/bin/python
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
total_code_chars = 0
total_str_chars = 0

for line in data:
    total_code_chars += len(line)
    total_str_chars += len(eval(line))

print total_code_chars - total_str_chars
