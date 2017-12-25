#!/usr/bin/python
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
packages = []
for line in data:
    packages.append(int(line))
#packages = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11]
print packages
balance_weight = sum(packages) / 3
min_passenger_packages = 1e100
min_passenger_packages_qe = 1e100
print "Balance weight: " + str(balance_weight)
def calculate_qe(packages):
    prod = 1
    for p in packages:
        prod *= p
    return prod
def partition_groups(packages, g1=[], g1sum=0, g2=[], g2sum=0, g3=[], g3sum=0):
    global balance_weight, min_passenger_packages, min_passenger_packages_qe
    if len(g1) > min_passenger_packages or (len(g1) == min_passenger_packages and calculate_qe(g1) >= min_passenger_packages_qe):
        if len(g2) > min_passenger_packages or (len(g2) == min_passenger_packages and calculate_qe(g2) >= min_passenger_packages_qe):
            if len(g3) > min_passenger_packages or (len(g3) == min_passenger_packages and calculate_qe(g3) >= min_passenger_packages_qe):
                return
    if g1sum > balance_weight or g2sum > balance_weight or g3sum > balance_weight:
        return
    if g1sum == g2sum == g3sum == balance_weight:
        if len(g1) < min_passenger_packages:
            min_passenger_packages = len(g1)
            min_passenger_packages_qe = calculate_qe(g1)
        elif len(g1) == min_passenger_packages:
            min_passenger_packages_qe = min(min_passenger_packages_qe, calculate_qe(g1))
        if len(g2) < min_passenger_packages:
            min_passenger_packages = len(g2)
            min_passenger_packages_qe = calculate_qe(g2)
        elif len(g2) == min_passenger_packages:
            min_passenger_packages_qe = min(min_passenger_packages_qe, calculate_qe(g2))
        if len(g3) < min_passenger_packages:
            min_passenger_packages = len(g3)
            min_passenger_packages_qe = calculate_qe(g3)
        elif len(g3) == min_passenger_packages:
            min_passenger_packages_qe = min(min_passenger_packages_qe, calculate_qe(g3))
        print g1, g2, g3, 'QE: ' + str(min_passenger_packages_qe)
    else:
        new_packages = list(packages)
        new_g1 = list(g1)
        new_g2 = list(g2)
        new_g3 = list(g3)
        p = new_packages.pop()
        new_g1.append(p)
        partition_groups(new_packages, g1=new_g1, g1sum=g1sum+p, g2=g2,
                g2sum=g2sum, g3=g3, g3sum=g3sum)
        new_g2.append(p)
        partition_groups(new_packages, g1=g1, g1sum=g1sum, g2=new_g2,
                g2sum=g2sum+p, g3=g3, g3sum=g3sum)
        new_g3.append(p)
        partition_groups(new_packages, g1=g1, g1sum=g1sum, g2=g2,
                g2sum=g2sum, g3=new_g3, g3sum=g3sum+p)
partition_groups(packages)
