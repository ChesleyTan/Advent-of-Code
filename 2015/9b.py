#!/usr/bin/python
from collections import namedtuple
from itertools import permutations
data = open(__file__[:-3]+ '.txt', 'r').read().split('\n')[:-1]
Node = namedtuple("Node", "name edges")
Edge = namedtuple("Edge", "dest cost")
nodes = {}

for line in data:
    line = line.split(" to ")
    line[1] = line[1].split(" = ")
    a = line[0]
    b = line[1][0]
    cost = int(line[1][1])
    if a not in nodes:
        nodes[a] = Node(name=a, edges=[])
    if b not in nodes:
        nodes[b] = Node(name=b, edges=[])
    nodes[a].edges.append(Edge(dest=b, cost=cost))
    nodes[b].edges.append(Edge(dest=a, cost=cost))

def edge_cost(src, dest):
    for edge in nodes[src].edges:
        if edge.dest == dest:
            return edge.cost

max_cost = -1e100
print nodes
for path in permutations(nodes.keys()):
    print max_cost
    print path
    cost = 0
    for i in xrange(len(path) - 1):
        cost += edge_cost(path[i], path[i+1])
    max_cost = max(max_cost, cost)
