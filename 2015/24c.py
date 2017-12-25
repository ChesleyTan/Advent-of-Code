#!/usr/bin/python
import operator
from itertools import combinations
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
packages = []
for line in data:
    packages.append(int(line))
#packages = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11]
print packages
balance_weight = sum(packages) / 4
min_passenger_packages = None
min_passenger_packages_qe = 1e100
print "Balance weight: " + str(balance_weight)
def calculate_qe(packages):
    return reduce(operator.mul, packages, 1)
#Assumptions: (1) There is a way to arrange the other groups so that the sums are balanced
for num_packages in xrange(1, len(packages)):
    if min_passenger_packages:
        break
    #for x in [c for c in combinations(packages, num_packages) if sum(c) == balance_weight]:
    #    min_passenger_packages = num_packages
    #    min_passenger_packages_qe = min(min_passenger_packages_qe, calculate_qe(x))
    #    print x, calculate_qe(x)
#Assumptions: (2) The first solution we encounter has the minimum quantum
#entanglement, due to the behavior of itertools.combinations()
    for x in (c for c in combinations(packages, num_packages) if sum(c) == balance_weight):
        min_passenger_packages = num_packages
        min_passenger_packages_qe = calculate_qe(x)
        print x, min_passenger_packages_qe
        break
print "Min QE: " + str(min_passenger_packages_qe)
