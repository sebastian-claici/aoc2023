from collections import defaultdict
from math import lcm


def neighbors(arr, i, j):
    result = []
    m, n = len(arr), len(arr[0])
    if i > 0:
        result.append((i - 1, j))
    if j > 0:
        result.append((i, j - 1))
    if i < m - 1:
        result.append((i + 1, j))
    if j < n - 1:
        result.append((i, j + 1))
    if i > 0 and j > 0:
        result.append((i - 1, j - 1))
    if i > 0 and j < n - 1:
        result.append((i - 1, j + 1))
    if i < m - 1 and j > 0:
        result.append((i + 1, j - 1))
    if i < m - 1 and j < n - 1:
        result.append((i + 1, j + 1))

    return result


def parse(filename):
    text = open(filename).readlines()
    inst = text[0].strip()

    edges = defaultdict(list)
    for line in text[2:]:
        a, b = line.split("=")
        b = b.strip()
        b, c = b[1:-1].split(",")
        a = a.strip()
        c = c.strip()
        edges[a].append(b)
        edges[a].append(c)

    return inst, edges


def solve(inst, edges, node):
    count = 0
    for _ in range(100):
        for i in inst:
            if node.endswith('Z'):
                return count
            count += 1
            if i == 'L':
                node = edges[node][0]
            else:
                node = edges[node][1]

    return -1


inst, edges = parse("day8.in")
nodes = [k for k in edges.keys() if k.endswith('A')]
counts = [solve(inst, edges, node) for node in nodes]

print(counts)
print(lcm(*counts))
