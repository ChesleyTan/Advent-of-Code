#!/usr/bin/python
row = 2981
col = 3075
def seq_sum(start, iters, stepsize):
    val = start
    for _ in xrange(iters):
        val += stepsize
        stepsize += 1
    return val
iterations = seq_sum(seq_sum(0, col, 1), row - 1, col)
print iterations
code = 20151125
for i in xrange(1, iterations):
    code = (code * 252533) % 33554393
print code
